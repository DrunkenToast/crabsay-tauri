<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Generator</ion-title>
                <ion-buttons slot="end">
                    <ion-button v-on:click="exportImg">
                        <ion-icon slot="start" :icon="save"></ion-icon>
                        Export image
                    </ion-button>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>

        <ion-content :fullscreen="true">
            <ion-header collapse="condense">
                <ion-toolbar>
                    <ion-title size="large">Tab 2</ion-title>
                </ion-toolbar>
            </ion-header>
            <ion-refresher slot="fixed" @ion-refresh="doRefresh($event)">
                <ion-refresher-content> </ion-refresher-content>
            </ion-refresher>
            <div class="center" v-if="isGenerating">
                <ion-spinner name="crescent"></ion-spinner>
            </div>
            <img class="generated" v-bind:src="imgDataUrl" />
        </ion-content>
    </ion-page>
</template>

<style>
.generated {
    display: block;
    max-height: 100%;
    margin: 1em;
    margin-left: auto;
    margin-right: auto;
}

.center {
    display: flex;
    justify-content: center;
}
</style>

<script lang="ts">
import { defineComponent, ref } from "vue";
import {
    IonPage,
    IonHeader,
    IonToolbar,
    IonTitle,
    IonContent,
    IonIcon,
    IonButton,
    IonButtons,
    IonRefresher,
    IonRefresherContent,
    IonSpinner,
} from "@ionic/vue";
import { save } from "ionicons/icons";
import Settings from "../services/settings";
import { createToast } from "../common/toast";
import { roundRect, drawPolygon, saveBlobUrl } from "../common/util";
import { Point } from "@/models/point";
import { Haptics } from "@capacitor/haptics";

const ImageLength = 3;
const ImagePaths = [
    "assets/img/chillin.jpg",
    "assets/img/blurrycow.jpg",
    "assets/img/sup.jpg",
];
const MouthPoints: Point[] = [
    { x: 2000, y: 1200 },
    { x: 700, y: 1800 },
    { x: 700, y: 1000 },
];

let imgDataUrl = ref("");
let canvas = document.createElement("canvas");
let img: HTMLImageElement | undefined;
let isGenerating = ref(false);

async function generateImage(): Promise<void> {
    isGenerating.value = true;
    imgDataUrl.value = "";
    return new Promise<void>((resolve, reject) => {
        let ctx = canvas.getContext("2d");
        if (!ctx) reject("no context");

        const index = Math.floor(Math.random() * ImageLength);

        img = new Image();
        img.src = process.env.BASE_URL + ImagePaths[index];
        img.onload = async () => {
            console.log("generating");
            if (!ctx) return;
            if (img == undefined) return;

            // const padding = img.width * 0.01;
            const aspectRatio = img.width / img.height;
            const padding = 5;
            const strokeSize = padding * 2;

            canvas.height = 2160;
            canvas.width = canvas.height * aspectRatio;

            ctx.imageSmoothingEnabled = true;
            ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

            const fontSize = 72;
            const lineHeight = fontSize * 1.286; //approximation
            const lines = Settings.text.split("\n");

            ctx.fillStyle = "#ffffff";
            ctx.strokeStyle = Settings.color;

            // Draw tail of bubble
            drawPolygon(
                ctx,
                [{ x: 20, y: 0 }, MouthPoints[index], { x: 200, y: 0 }],
                strokeSize
            );

            // Draw bubble
            roundRect(
                ctx,
                padding,
                padding,
                canvas.width - padding * 2,
                // canvas.height - padding * 2,
                lines.length * lineHeight,
                20,
                strokeSize
            );

            // Draw text on bubble
            ctx.font = fontSize + "px Comic Sans MS";
            ctx.fillStyle = Settings.color;
            for (let i = 0; i < lines.length; i++) {
                ctx.fillText(
                    lines[i],
                    padding + 15,
                    padding + 72 + lineHeight * i
                );
            }

            canvas.toBlob((b) => {
                if (!b) {
                    reject("No blob was generated");
                    return;
                }
                const url = URL.createObjectURL(b);

                imgDataUrl.value = url;
                isGenerating.value = false;
                resolve();
            }, "image/png");

            // imgDataUrl.value = canvas.toDataURL();
        };
    });
}

export default defineComponent({
    name: "GeneratorsPage",
    components: {
        IonHeader,
        IonToolbar,
        IonTitle,
        IonContent,
        IonPage,
        IonIcon,
        IonButton,
        IonButtons,
        IonRefresher,
        IonRefresherContent,
        IonSpinner,
    },
    ionViewDidEnter() {
        if (isGenerating.value) return;
        generateImage().then(async () => {
            await Haptics.vibrate();
        });
    },
    data() {
        return {
            Settings,
            imgDataUrl,
            isGenerating,
        };
    },
    methods: {},
    setup() {
        const exportImg = (_event: any) => {
            saveBlobUrl(imgDataUrl.value).then((s: string) =>
                createToast("Image exported to: " + s)
            );
        };
        const doRefresh = (event: any) => {
            generateImage().finally(() => {
                event.target.complete();
                Haptics.vibrate();
            });
        };
        return { save, doRefresh, exportImg };
    },
});
</script>
