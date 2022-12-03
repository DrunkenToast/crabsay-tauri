<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Creator</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content :fullscreen="true">
            <ion-header collapse="condense">
                <ion-toolbar>
                    <ion-title size="large">Tab 1</ion-title>
                </ion-toolbar>
            </ion-header>
            <div class="container">
                <h1>From rust: {{greet}}</h1>
                <ion-list>
                    <ion-item>
                        <ion-label position="stacked">Cowsay speech</ion-label>
                        <ion-textarea
                            placeholder="Big epic speech"
                            v-model="Settings.text"
                            :auto-grow="true"
                        >
                        </ion-textarea>
                    </ion-item>

                    <ion-item>
                        <ion-label>Pick a color!</ion-label>
                        <input
                            name="color"
                            type="color"
                            v-model="Settings.color"
                        />
                    </ion-item>

                    <ion-item>
                        <ion-checkbox slot="start"></ion-checkbox>
                        <ion-label>Darkmode!!!</ion-label>
                    </ion-item>
                </ion-list>
                <ion-button router-link="/generator">Generate</ion-button>
            </div>
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import {
    IonPage,
    IonHeader,
    IonToolbar,
    IonTitle,
    IonContent,
    IonList,
    IonItem,
    IonLabel,
    IonInput,
    IonButton,
    IonCheckbox,
    IonTextarea,
} from "@ionic/vue";
import Settings from "../services/settings";
import { invoke } from '@tauri-apps/api';

let greet = ref('');

export default defineComponent({
    name: "Tab1Page",
    components: {
        IonHeader,
        IonToolbar,
        IonTitle,
        IonContent,
        IonPage,
        IonList,
        IonItem,
        IonLabel,
        IonButton,
        IonCheckbox,
        IonTextarea,
    },
    ionViewDidEnter() {
        invoke('greet', { name: 'this text was sent from vue!'})
        .then((msg) => {
                if (typeof msg === 'string')
                greet.value = msg
            })
        .catch((e) => console.log(e))
    },
    data() {
        return {
            Settings,
            greet
        };
    },
});
</script>

<style>
.container {
    display: flex;
    flex-direction: column;
    margin: auto;
}

@media screen and (min-width: 768px) {
    .container {
        width: 50%;
    }
}

input {
    padding: 0;
}
</style>
