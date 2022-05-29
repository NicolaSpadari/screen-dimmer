<template>
    <div bg-black w-screen h-screen transition-opacity duration-750 :class="`opacity-${res}`" />
</template>

<script lang="ts" setup>
    const res = ref(0);

    let unlisten: () => void;

    onMounted(async () => {
        setTimeout(() => {
            res.value = 20;
        }, 1000);

        unlisten = await listen("opacity", (data: TauriEvent) => {
            res.value = Number(data.payload.message);
        });
    });

    onBeforeUnmount(() => unlisten());
</script>
