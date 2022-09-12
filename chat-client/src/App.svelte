<script lang="ts">
  import sanitizeHtml from 'sanitize-html';
  import {MessageStore} from "./lib/messageStore";
  import Message from "./lib/Message.svelte";
  import type {ChatMessage} from "./lib/MessageTypes";

  let listRef;

  function scrollToBottom() {
    listRef?.scrollIntoView({
      behavior: "smooth"
    })
  }

  let socket = new WebSocket('ws://localhost:9002');
  socket.onmessage = (soc) => {
    const input: ChatMessage = JSON.parse(soc.data)
    const cleanMessage = sanitizeHtml(input.message)
    $MessageStore = [...$MessageStore, {user: input.user, message: cleanMessage}]
    scrollToBottom()
  }

  socket.onerror = () => {
    socket.close();
    socket = new WebSocket('ws://localhost:9002');
  }

  socket.onclose = (ev) => {
    console.error(ev.reason)
  }


</script>

<main>
    <div>
        <ul>


            {#each $MessageStore as messageData}
                <li bind:this={listRef}>
                    <Message message={messageData.message} user={messageData.user}/>
                </li>
            {/each}
        </ul>

    </div>
</main>

<style>

    ul {
        display: inline-block;
        text-align: left;
        list-style: none;
    }
</style>