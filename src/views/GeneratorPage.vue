<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Generator</ion-title>
                <ion-buttons slot="end">
                    <ion-button v-bind:href="imgDataUrl" download="cowsay4k.png">
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
                <ion-refresher-content>
                </ion-refresher-content>
            </ion-refresher>
                    <img class="generated" alt="Generated image" v-bind:src="imgDataUrl" />
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
</style>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { IonPage, IonHeader, IonToolbar, IonTitle, IonContent, IonIcon, IonButton, IonButtons, IonRefresher, IonRefresherContent } from '@ionic/vue';
import { save } from 'ionicons/icons';
import Settings from '../services/settings';
import { roundRect, drawPolygon } from '../common/util';
import { Point } from "@/models/point";
import { Haptics } from '@capacitor/haptics';

const ImageLength = 3;
const ImagePaths = [
    "assets/img/chillin.jpg",
    "assets/img/blurrycow.jpg",
    "assets/img/sup.jpg"
];
const MouthPoints: Point[] = [
    {x: 2000, y: 1200}, {x: 700, y: 1800}, {x: 700, y: 1000}
];

let imgDataUrl = ref('');
let canvas = document.createElement('canvas');
let img: HTMLImageElement | undefined;

function generateImage(): Promise<void> {
    return new Promise<void>((resolve, reject) => {
        let ctx = canvas.getContext('2d');
        if (!ctx) reject('no context');

        const index = Math.floor(Math.random() * ImageLength);

        img = new Image();
        img.src = process.env.BASE_URL + ImagePaths[index];
        img.onload = async () => {
            console.log('generating')
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
            const lineHeight = fontSize * 1.286 //approximation
            const lines = Settings.text.split('\n');

            ctx.fillStyle = "#ffffff";
            ctx.strokeStyle = Settings.color;

            // Draw tail of bubble 
            drawPolygon(ctx, [
                {x: 20, y: 0},
                MouthPoints[index],
                {x: 200, y: 0},
            ], strokeSize)

            // Draw bubble
            roundRect(ctx, padding, padding, 
                canvas.width - padding * 2, 
                // canvas.height - padding * 2, 
                lines.length * lineHeight,
                20,
                strokeSize
            );

            // Draw text on bubble
            ctx.font = fontSize + "px Comic Sans MS"
            ctx.fillStyle = Settings.color;
            for (let i = 0; i < lines.length; i++) {
                ctx.fillText(lines[i], padding + 15, padding+ 72 + lineHeight*i)         
            }

            imgDataUrl.value = canvas.toDataURL();
            resolve();
        }

    })
}


export default defineComponent({
    name: 'GeneratorsPage',
    components: { IonHeader, IonToolbar, IonTitle, IonContent, IonPage, IonIcon, IonButton, IonButtons, IonRefresher, IonRefresherContent },
    ionViewWillEnter() {
        imgDataUrl.value = '';
        generateImage()
            .then(async () => await Haptics.vibrate());
        console.log('here');
    },
    data() {
        return {
        Settings,
        imgDataUrl,
    }
},
    setup() {
        const doRefresh = (event: any) => {
            generateImage()
                .finally(() => {
                    event.target.complete();
                    Haptics.vibrate();
                });

        }
        return { save, doRefresh }
    }
    });
</script>
