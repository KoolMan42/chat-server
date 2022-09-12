import {createClient, FetchTransport, WebsocketTransport} from '@rspc/client';
import type {Operations} from "./bindings";

// For fetch transport
export const client = createClient<Operations>({
  transport: new FetchTransport("http://localhost:9001/rspc"),
});

// For websocket transport - Required for subscriptions
export const clientWS = createClient<Operations>({
  transport: new WebsocketTransport("ws://localhost:9001/rspcws"),
});
