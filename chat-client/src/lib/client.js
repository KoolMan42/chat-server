import { createClient, FetchTransport, WebsocketTransport } from '@rspc/client';
// For fetch transport
export const client = createClient({
    transport: new FetchTransport("http://localhost:9001/rspc"),
});
// For websocket transport - Required for subscriptions
export const clientWS = createClient({
    transport: new WebsocketTransport("ws://localhost:9001/rspcws"),
});
//# sourceMappingURL=client.js.map