<script lang="ts">


  import type {User} from "./MessageTypes";

  export let message: string;
  export let user: User
</script>

<div>
  <span style="color: {user.color}">{user.username}</span>: {message}
</div>


<style>



</style>