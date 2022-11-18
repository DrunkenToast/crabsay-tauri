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
import { createToast } from "../util/toast";
import saveBlobUrl from "../util/save-image";
import Haptics from "../util/haptics";
import { generateImage } from "../util/image-generation";

let imgDataUrl = ref("");
let canvas = document.createElement("canvas");
let img: HTMLImageElement | undefined;
let isGenerating = ref(false);

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
        generateImage(isGenerating, imgDataUrl, canvas, img).then(async () => {
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
            generateImage(isGenerating, imgDataUrl, canvas, img).finally(() => {
                event.target.complete();
                Haptics.vibrate();
            });
        };
        return { save, doRefresh, exportImg };
    },
});
</script>
