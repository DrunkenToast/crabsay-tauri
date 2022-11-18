import { toastController } from "@ionic/vue";

type Position = 'top' | 'middle' | 'bottom';

export async function createToast(msg: string, position: Position = 'bottom') {
    const toast = await toastController.create({
        message: msg,
        duration: 1000,
        position: position,
    });
    console.log('test')

    await toast.present();
}
