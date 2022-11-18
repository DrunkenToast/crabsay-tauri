import { Capacitor } from "@capacitor/core";
import { Directory, Filesystem } from "@capacitor/filesystem";

function generateFileName(): string {
    return new Date().getTime() + "-cowsay.png";
}

// Mock/fallback
const saveBlobUrlCompat = async (blobUrl: string): Promise<string> => {
    const blob = await fetch(blobUrl).then(r => r.blob());
    const dataUrl = await blobToBase64(blob);

    const link = document.createElement('a');
    const fileName = generateFileName();
    link.setAttribute('download', fileName);
    link.setAttribute('href', dataUrl);

    link.click();
    return fileName;
}

// Native
const saveBlobUrlNative = async (blobUrl: string): Promise<string> => {
    const blob = await fetch(blobUrl).then(r => r.blob());
    const dataUrl = await blobToBase64(blob);
    const data = dataUrl.replace(/^data:image\/\w+;base64,/, "");

    const file = await Filesystem.writeFile({
        path: generateFileName(),
        data: data,
        directory: Directory.Documents,
    });
    return file.uri;
}

async function blobToBase64(blob: any): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(blob);
        reader.onload = () => resolve(reader.result as string);
        reader.onerror = (error) => reject(error);
    });
}

// Factory
let saveBlobUrl: (url: string) => Promise<string>;
if (Capacitor.isNativePlatform()) {
    saveBlobUrl = saveBlobUrlNative;
}
else {
    saveBlobUrl = saveBlobUrlCompat;
}

export default saveBlobUrl;
