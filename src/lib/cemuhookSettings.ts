export const DEFAULT_CEMUHOOK_ADDRESS = "127.0.0.1";
export const DEFAULT_CEMUHOOK_PORT = 26760;

export function validateCemuHookSettings(address: string, port: number): string | null {
    const ip = address.trim();
    const ipv4 = ip.split(".");
    const isIpv4 = ipv4.length === 4 && ipv4.every(part => /^(0|[1-9]\d{0,2})$/.test(part) && Number(part) <= 255);
    let isIpv6 = false;
    if (ip.includes(":") && !/[\[\]\s/%]/.test(ip)) {
        try { isIpv6 = new URL(`http://[${ip}]/`).hostname.startsWith("["); } catch { /* Invalid IPv6 literal. */ }
    }
    if (!isIpv4 && !isIpv6) return "Enter an IPv4 or IPv6 bind address on this PC.";
    if (!Number.isInteger(port) || port < 1 || port > 65535) return "CemuHook port must be an integer between 1 and 65535.";
    return null;
}
