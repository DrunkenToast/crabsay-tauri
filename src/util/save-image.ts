import Settings from "../services/settings";
import { invoke } from "@tauri-apps/api";
import { save } from '@tauri-apps/api/dialog';
import { createToast } from "./toast";

async function exportImage() {
    const filePath = await save({
        filters: [{
            name: 'Image',
            extensions: ['png']
        }]
    });
    
    if (!filePath) return;

    invoke("save_image", {
        writePath: filePath,
        message: Settings.text.value,
        color: Settings.color.value,
    })
        .then(() => {
            createToast("Image exported to: " + filePath, "top");
        })
        .catch((e) => {
            createToast(
                "An error occured exporting your image"
            );
            console.error(e);
        });
}

export default exportImage;
