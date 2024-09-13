
<script lang="ts">
    let password = "";
    let showPassword = false;

    $: password ="";

  </script>
  
  <style>
        .container {
      background-color: white;
      border-radius: 8px;
      padding: 40px;
      width: var(--login-width);
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      text-align: center;
      margin: auto;
    }
    .password-input {
      width: 100%;
      padding: 0.5rem;
      height: 2.5rem;
      margin-bottom: 2%;
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
      width: 60%;
    }
    .send-btn {
      background-color: #6a0dad;
      color: white;
      border: none;
      border-radius: 3px;
      cursor: pointer;
      font-size: 14px;
    }
    .password{
        position: relative;
    }
    .show-button{
        position: absolute;
        right: 2%;
        top: 25%;
        text-decoration: underline;
    }
  </style>
  
  <div class="container">
    <div class="title text-left font-sans text-xl text-red-400 m-5 ">
        <span class="basis-8/10 ml-2 pb-0">Reset your password</span>
    </div>
    <div class="mt-4">
        <div class="password m-4 flex flex-row">
            <div class="basis-4/5">
                <input type="email" placeholder="Email" class="password-input">
            </div>
            <div class="basis-1/5">
                <button class="send-btn px-3 py-2 h-10 ml-3">Send</button>
            </div>
        </div>
        <div>
            
        </div>
        <div class="password m-4">
            {#if showPassword}
            <input
              type="text"
              class="password-input"
              placeholder="Password"
              bind:value={password}
            />
          {:else}
            <input
              type="password"
              class="password-input"
              placeholder="Password"
              bind:value={password}
            />
          {/if}
          <button on:click={() => (showPassword = !showPassword)} class="show-button">
            {showPassword ? "Hide" : "Show"}
          </button>
        </div>
        <!-- <PasswordVarify bind:password={password}></PasswordVarify> -->
    </div>
    <button class="continue-btn my-4">Continue</button>
    <!-- <div class="forgot-password">Forgot password?</div> -->
  </div>
  