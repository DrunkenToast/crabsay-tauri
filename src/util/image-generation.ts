import Settings from "../services/settings";
import { roundRect, drawPolygon } from "../util/draw";
import { Point } from "@/models/point";
import { Ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { createToast } from "./toast";

export async function generateImage(
    isGenerating: Ref<boolean>,
    imgDataUrl: Ref<string>
): Promise<void> {
    return new Promise<void>((resolve, reject) => {
        isGenerating.value = true;
        imgDataUrl.value = "";

        invoke("generate_image", {
            message: Settings.text.value,
            color: Settings.color.value,
        })
            .then((path) => {
                console.log("Generated path: ", path);
                // Prevent caching of image with timestamp in url to make it unique
                const timestamp = Date.now();
                imgDataUrl.value =
                    convertFileSrc(path as string) + "?t=" + timestamp;
            })
            .catch((e) => {
                createToast(
                    "An error occured while generating an image: " + e,
                    "middle"
                );
                console.error(e);
            })
            .finally(() => {
                isGenerating.value = false;
                resolve();
            });
    });
}
