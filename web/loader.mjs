const SAMPLE_WINDOW_MS = 5_000;
const MIN_RATE_DURATION_MS = 250;

export function formatBytes(bytes) {
    if (!Number.isFinite(bytes) || bytes <= 0) {
        return "0 B";
    }

    const units = ["B", "KiB", "MiB", "GiB"];
    const unitIndex = Math.min(
        Math.floor(Math.log(bytes) / Math.log(1024)),
        units.length - 1,
    );
    const value = bytes / 1024 ** unitIndex;
    const precision = unitIndex === 0 || value >= 100 ? 0 : 1;

    return `${value.toFixed(precision)} ${units[unitIndex]}`;
}

export function formatDuration(seconds) {
    if (!Number.isFinite(seconds) || seconds < 0) {
        return "未知";
    }

    const roundedSeconds = Math.ceil(seconds);
    if (roundedSeconds < 60) {
        return `${roundedSeconds} 秒`;
    }

    const minutes = Math.floor(roundedSeconds / 60);
    const remainingSeconds = roundedSeconds % 60;
    return `${minutes} 分 ${remainingSeconds} 秒`;
}

export function calculateTransferRate(samples) {
    if (samples.length < 2) {
        return 0;
    }

    const first = samples[0];
    const last = samples[samples.length - 1];
    const duration = last.time - first.time;
    const transferred = last.bytes - first.bytes;

    if (duration < MIN_RATE_DURATION_MS || transferred <= 0) {
        return 0;
    }

    return (transferred * 1_000) / duration;
}

export function estimateRemainingSeconds(current, total, bytesPerSecond) {
    if (total <= 0 || bytesPerSecond <= 0 || current > total) {
        return null;
    }

    return (total - current) / bytesPerSecond;
}

export function normalizeProgress(current, callbackTotal, responseTotal) {
    const total = responseTotal > 0 ? responseTotal : callbackTotal;
    return {
        current: responseTotal > 0 ? Math.min(current, responseTotal) : current,
        total,
    };
}

function requiredElement(id) {
    const element = document.getElementById(id);
    if (!element) {
        throw new Error(`Missing loading screen element: #${id}`);
    }
    return element;
}

function loadingElements() {
    return {
        screen: requiredElement("loading-screen"),
        status: requiredElement("loading-status"),
        progress: requiredElement("loading-progress"),
        percent: requiredElement("loading-percent"),
        bytes: requiredElement("loading-bytes"),
        speed: requiredElement("loading-speed"),
        remaining: requiredElement("loading-remaining"),
        error: requiredElement("loading-error"),
        retry: requiredElement("loading-retry"),
    };
}

function requestWasmSize() {
    const wasmPreload = document.querySelector(
        'link[rel="preload"][as="fetch"][type="application/wasm"]',
    );
    if (!wasmPreload) {
        return Promise.resolve(0);
    }

    return fetch(wasmPreload.href, { headers: { Range: "bytes=0-0" } })
        .then((response) => {
            if (!response.ok) {
                return 0;
            }

            const contentRange = response.headers.get("Content-Range");
            const rangeTotal = Number(contentRange?.match(/\/(\d+)$/)?.[1]);
            const contentLength = Number(response.headers.get("Content-Length"));
            const responseIsEncoded = response.headers.has("Content-Encoding");
            response.body?.cancel();

            if (Number.isFinite(rangeTotal) && rangeTotal > 0) {
                return rangeTotal;
            }
            return !responseIsEncoded && Number.isFinite(contentLength) && contentLength > 0
                ? contentLength
                : 0;
        })
        .catch(() => 0);
}

export default function createLoader() {
    const elements = loadingElements();
    const samples = [];
    let responseTotal = 0;

    requestWasmSize().then((total) => {
        responseTotal = total;
    });

    elements.retry.addEventListener("click", () => window.location.reload());

    function updateProgress(callbackCurrent, callbackTotal) {
        const { current, total } = normalizeProgress(
            callbackCurrent,
            callbackTotal,
            responseTotal,
        );
        const now = performance.now();
        samples.push({ time: now, bytes: current });

        const cutoff = now - SAMPLE_WINDOW_MS;
        while (samples.length > 2 && samples[1].time < cutoff) {
            samples.shift();
        }

        const rate = calculateTransferRate(samples);
        elements.bytes.textContent = total > 0
            ? `${formatBytes(current)} / ${formatBytes(total)}`
            : formatBytes(current);
        elements.speed.textContent = rate > 0
            ? `${formatBytes(rate)}/秒`
            : "计算中…";

        if (total <= 0) {
            elements.progress.removeAttribute("value");
            elements.percent.textContent = "—";
            elements.remaining.textContent = "未知";
            elements.status.textContent = "正在下载游戏资源…";
            return;
        }

        const percentage = Math.min(100, Math.max(0, (current / total) * 100));
        elements.progress.value = percentage;
        elements.progress.textContent = `${Math.round(percentage)}%`;
        elements.progress.setAttribute("aria-valuetext", `${Math.round(percentage)}%`);
        elements.percent.textContent = `${Math.round(percentage)}%`;

        const remaining = estimateRemainingSeconds(current, total, rate);
        elements.remaining.textContent = remaining === null
            ? "计算中…"
            : formatDuration(remaining);
        elements.status.textContent = percentage >= 100
            ? "下载完成，正在启动游戏…"
            : "正在下载游戏资源…";
    }

    return {
        onStart: () => {
            elements.status.textContent = "正在连接服务器…";
            elements.error.hidden = true;
            elements.retry.hidden = true;
        },
        onProgress: ({ current, total }) => updateProgress(current, total),
        onComplete: () => {},
        onSuccess: () => {
            elements.status.textContent = "游戏已就绪";
            elements.progress.value = 100;
            elements.progress.textContent = "100%";
            elements.percent.textContent = "100%";
            elements.screen.setAttribute("aria-hidden", "true");
            document.body.classList.add("app-ready");

            const removeScreen = () => elements.screen.remove();
            elements.screen.addEventListener("transitionend", (event) => {
                if (event.target === elements.screen && event.propertyName === "opacity") {
                    removeScreen();
                }
            });
            window.setTimeout(removeScreen, 400);
        },
        onFailure: (error) => {
            console.error("HexGo web initialization failed.", error);
            elements.status.textContent = "无法启动游戏";
            elements.progress.removeAttribute("value");
            elements.percent.textContent = "—";
            elements.remaining.textContent = "未知";
            elements.error.hidden = false;
            elements.retry.hidden = false;
            elements.retry.focus();
        },
    };
}
