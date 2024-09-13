<script>
    import Middle from "$lib/component/home/middle.svelte";
    import Header from "$lib/component/home/header.svelte";
    import Bottom from "$lib/component/home/bottom.svelte";
</script>

<div class="containers">
    <Header />
    <Middle />
    <Bottom class="bottom" />
</div>

<style>
    .containers {
        display: flex; /* 使容器成为一个 Flexbox */
        flex-direction: column; /* 子元素垂直排列 */
        justify-content: space-between; /* 使得 Header 和 Bottom 分布在容器的两端 */
        height: 814px; /* 设置容器的高度 */
        position: relative; /* 为了将内容相对定位 */
    }

    .bottom {
        margin-top: auto; /* 将 Bottom 组件推到底部 */
    }
</style>
