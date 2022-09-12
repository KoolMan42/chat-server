import {writable} from "svelte/store";
import type {ChatMessage} from "./MessageTypes";




export const MessageStore = writable<Array<ChatMessage>>([])