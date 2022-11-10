import { Point } from "@/models/point";
import { Directory, Filesystem } from "@capacitor/filesystem";

interface Radius {
    tl: number;
    tr: number;
    br: number;
    bl: number;
}

export function roundRect(
    context: CanvasRenderingContext2D,
    x: number,
    y: number,
    w: number,
    h: number,
    radius: number | Radius,
    lineWidth: number
) {
    if (typeof radius == "number") {
        radius = {
            tl: radius,
            tr: radius,
            br: radius,
            bl: radius,
        };
    }

    context.beginPath();

    context.lineWidth = lineWidth;

    context.moveTo(x + radius.tl, y);
    context.lineTo(x + w - radius.tr, y);
    context.quadraticCurveTo(x + w, y, x + w, y + radius.tr);
    context.lineTo(x + w, y + h - radius.br);
    context.quadraticCurveTo(x + w, y + h, x + w - radius.br, y + h);
    context.lineTo(x + radius.bl, y + h);
    context.quadraticCurveTo(x, y + h, x, y + h - radius.bl);
    context.lineTo(x, y + radius.tl);
    context.quadraticCurveTo(x, y, x + radius.tl, y);
    context.closePath();

    context.fill();
    context.stroke();
}

export function drawPolygon(
    context: CanvasRenderingContext2D,
    points: Point[],
    lineWidth: number
) {
    context.beginPath();

    context.lineWidth = lineWidth;

    context.moveTo(points[0].x, points[0].y);

    for (let i = 1; i < points.length; i++) {
        context.lineTo(points[i].x, points[i].y);
    }

    context.closePath();

    context.fill();
    context.stroke();
}

export async function saveBlobUrl(blobUrl: string): Promise<string> {
    const blob = await fetch(blobUrl).then(r => r.blob());
    const dataUrl = await blobToBase64(blob);
    const data = dataUrl.replace(/^data:image\/\w+;base64,/, "");

    console.error("yes!", data);

    const fileName = new Date().getTime() + "-cowsay.png";
    const savedFile = await Filesystem.writeFile({
        path: fileName,
        data: data,
        directory: Directory.Documents,
    });
    return savedFile.uri;
}

async function blobToBase64(blob: any): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(blob);
        reader.onload = () => resolve(reader.result as string);
        reader.onerror = (error) => reject(error);
    });
}
