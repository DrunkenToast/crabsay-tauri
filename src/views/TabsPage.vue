<template>
    <ion-page>
        <ion-tabs>
            <ion-router-outlet></ion-router-outlet>
            <ion-tab-bar slot="bottom">
                <ion-tab-button tab="tab1" href="/creator">
                    <ion-icon :icon="flask" />
                    <ion-label>Creator</ion-label>
                </ion-tab-button>

                <ion-tab-button tab="tab2" href="/generator">
                    <ion-icon :icon="rocket" />
                    <ion-label>Generator</ion-label>
                </ion-tab-button>
            </ion-tab-bar>
        </ion-tabs>
    </ion-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { IonTabBar, IonTabButton, IonTabs, IonLabel, IonIcon, IonPage, IonRouterOutlet } from '@ionic/vue';
import { rocket, flask } from 'ionicons/icons';

export default defineComponent({
    name: 'TabsPage',
    components: { IonLabel, IonTabs, IonTabBar, IonTabButton, IonIcon, IonPage, IonRouterOutlet },
    setup() {
        return {
            rocket,
            flask,
        }
    }
});
</script>
