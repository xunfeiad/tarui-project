<script lang="ts">
    import key from "$lib/assets/svg/key.svg";
    import email from "$lib/assets/svg/email.svg";
    import unlock from "$lib/assets/svg/unlock.svg";
    import lock from "$lib/assets/svg/lock.svg";

    export let inputStyle: string = " w-full rounded px-12 h-9 leading-9 mt-2.5 my-5 divide-solid border border-[#ddd]"

    export let transition = "transition ease-in-out delay-150 hover:-translate-y-1 hover:bg-purple-300 duration-300"

    export let loading = true;
</script>

<div class="container">
    <div class="title text-left">
        <span class="text-left font-sans text-xl text-red-400">Reigster</span>
      </div>
      
      <div class="email register-input align-middle text-center	">
        <img src="{email}" alt="" class="icon">
        <input type="email" name="email" id="email"  class="{inputStyle}" placeholder="Please input your email"/>
      </div>
      
      <div class="verify-code register-input">
        <img src="{key}" alt="" class="icon">
        <div class="flex flex-row">
            <input type="text" name="code" id="code" class="{inputStyle} basis-4/5" placeholder="Please input your code"/>
            <button class="mt-3 content-center basis-1/5 h-8 ml-3 bg-purple-600 text-white rounded  leading-4 {transition} flex justify-center items-center">
            
                {#if loading === false}
                Send
                {:else}
                    <svg class="animate-spin h-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                {/if}
            </button>
        </div>
      </div>
      <div class="password register-input">
        <img src="{lock}" alt="" class="icon">
        <input type="password" name="password" id="password" class="{inputStyle}" placeholder="Please input new password"/>
      </div>

      <div class="confirm-password register-input">
        <img src="{unlock}" alt="" class="icon">
        <input type="password" name="confirm-password" id="confirm-password" class="{inputStyle}" placeholder="Please confirm password"/>
      </div>
      
      <div class="flex justify-center mt-2">
        <button class="bg-purple-600 text-white rounded leading-9 w-1/3 h-9 {transition} flex flex-row justify-center items-center ml-0">
            {#if loading === false}
            Reigster
            {:else}
                <svg class="animate-spin h-5 w-5 ml-2 text-white basis-1/5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                 <span class="basis-4/5">Loading...</span>
            {/if}
            
        </button>
      </div>
</div>

<style>
    input:focus{
        outline: none;
    }

    .container {
      background-color: white;
      border-radius: 8px;
      padding: 20px;
      width: var(--auth-width);
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      text-align: center;
      margin: auto;
    }

    .register-input{
        position: relative;
    }
    .icon{
        width: 1.75rem;
        height: 1.75rem;
        position: absolute;
        top:0.9rem;
        left: 0.25rem;
    }
</style>
