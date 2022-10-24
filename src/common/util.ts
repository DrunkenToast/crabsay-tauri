import { Point } from "@/models/point";

interface Radius {
    tl: number,
    tr: number,
    br: number,
    bl: number,
}

export function roundRect(
    context: CanvasRenderingContext2D,
    x: number, y: number,
    w: number, h: number,
    radius: number | Radius,
    lineWidth: number
) {
    if (typeof (radius) == 'number') {
        radius = {
            tl: radius,
            tr: radius,
            br: radius,
            bl: radius,
        }
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
