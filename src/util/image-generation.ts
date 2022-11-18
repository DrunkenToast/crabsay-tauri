import Settings from "../services/settings";
import { roundRect, drawPolygon} from "../util/draw";
import { Point } from "@/models/point";
import { Ref } from "vue";

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

export async function generateImage(
    isGenerating: Ref<boolean>,
    imgDataUrl: Ref<string>,
    canvas: HTMLCanvasElement,
    img: HTMLImageElement | undefined
): Promise<void> {
    isGenerating.value = true;
    imgDataUrl.value = "";
    return new Promise<void>((resolve, reject) => {
        const ctx = canvas.getContext("2d");
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
            const lines = Settings.text.value.split("\n");

            ctx.fillStyle = "#ffffff";
            ctx.strokeStyle = Settings.color.value;

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
            ctx.fillStyle = Settings.color.value;
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
        };
    });
}
