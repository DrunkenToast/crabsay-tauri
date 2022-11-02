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
                <ion-list>
                    <ion-item>
                        <ion-label position="stacked">Cowsay speech{{Settings.text}}</ion-label>
                        <ion-textarea placeholder="Big epic speech"
                            v-model="Settings.text"
                            :auto-grow="true">
                        </ion-textarea>
                    </ion-item>

                    <ion-item>
                        <ion-label>Pick a color!</ion-label>
                        <input name="color" type="color" v-model="Settings.color"/>
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
import { defineComponent } from 'vue';
import { IonPage, IonHeader, IonToolbar, IonTitle, IonContent, IonList, IonItem, IonLabel, IonInput, IonButton, IonCheckbox, IonTextarea } from '@ionic/vue';
import Settings from '../services/settings'

export default  defineComponent({
    name: 'Tab1Page',
    components: { IonHeader, IonToolbar, IonTitle, IonContent, IonPage, IonList, IonItem, IonLabel, IonButton, IonCheckbox, IonTextarea },
    data() {
        return {
            Settings
        }
    },
});
</script>

<style>
.container {
    display: flex;
    flex-direction: column;
    margin: auto;
}

@media screen and (min-width: 	768px) {
    .container {
        width:50%;
    }
}

input {
    padding: 0;
}
</style>

