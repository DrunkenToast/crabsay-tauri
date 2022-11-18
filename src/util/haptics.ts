import { Capacitor } from "@capacitor/core";
import { Haptics as HapticsNative, HapticsPlugin, VibrateOptions } from "@capacitor/haptics";
import { createToast } from "./toast";

// Mock
export class HapticsMock {
    vibrate(_opt?: VibrateOptions) {
        console.log('Vibrating on device');
        createToast('Brrrrr');
        return Promise.resolve();
    }
}

// Factory
let Haptics: HapticsPlugin | HapticsMock;
if (Capacitor.isNativePlatform()) {
    Haptics = HapticsNative;
}
else {
    Haptics = new HapticsMock();
}

export default Haptics;
