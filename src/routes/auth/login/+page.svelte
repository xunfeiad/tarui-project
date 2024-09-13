<script lang="ts">
    import {goto} from "$app/navigation";
    import {login} from "$lib/api/auth/user";
    import {fade} from "svelte/transition";
    import {fa} from "@faker-js/faker";

    let showPassword = false;

    let user = {
        username: "",
        password: "",
    };

    let loading = false;

    $: user;
    $: loading;

    const handleClick = async (event) => {
        console.log(event)
        loading = true;
        const res = await login(user);
        loading = false;
        if (res.status == 200) {
            goto("/");
        }
    };
</script>

<div class="container" transition:fade>
    <div class="text-left">
        <span class="text-red-400/85">Sign in Redog</span>
    </div>
    <div>
        <input
                type="text"
                placeholder="Username"
                class="password-input"
                bind:value={user.username}
        />
    </div>
    <div class="password">
        {#if showPassword}
            <input
                    type="text"
                    class="password-input"
                    placeholder="Password"
                    bind:value={user.password}
            />
        {:else}
            <input
                    type="password"
                    class="password-input"
                    placeholder="Password"
                    bind:value={user.password}
            />
        {/if}
        <button on:click={() => (showPassword = !showPassword)} class="show-button">
            {showPassword ? "Hide" : "Show"}
        </button>
    </div>
    <button class="continue-btn" on:click={handleClick}>Login</button>
    <div class="forgot-password">Forgot password?</div>
</div>

<style>
    .container {
        background-color: white;
        border-radius: 8px;
        padding: 40px;
        width: var(--auth-width);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        text-align: center;
        margin: auto;
    }

    .password-input {
        width: 100%;
        padding: 10px;
        margin-top: 10px;
        margin-bottom: 20px;
        border-radius: 5px;
        border: 1px solid #ddd;
    }

    .continue-btn {
        background-color: #6a0dad;
        color: white;
        padding: 10px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        font-size: 16px;
        width: 100%;
    }

    .forgot-password {
        margin-top: 20px;
        font-size: 14px;
        color: #6a0dad;
    }

    .password {
        position: relative;
    }

    .show-button {
        position: absolute;
        right: 2%;
        top: 25%;
        text-decoration: underline;
    }
</style>
