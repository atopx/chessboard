<script setup lang="ts">
import { LogInst, NCard, NDivider, NFlex, NLog, NText } from 'naive-ui';
import { nextTick, onMounted, ref, watchEffect } from 'vue';


const log = `
炮物品二
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
色粉我方
`

const logRef = ref(log)
const logInstRef = ref<LogInst | null>(null)


onMounted(() => {
    watchEffect(() => {
        if (logRef.value) {
            nextTick(() => {
                logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
            })
        }
    })
})

const fenType = ref("error")

</script>

<template>
    <n-card title="局面分析" :bordered="false" class="textlog" content-style="color: blue">
        <n-flex justify="space-between" align="end">
            <n-text type="info" class="analyse-title" strong>
                马二进三
            </n-text>

            <n-text :type="fenType">
                -42
            </n-text>
            <n-text type="warning">
                -42
            </n-text>
        </n-flex>
        <n-divider />
        <n-log :rows=18 ref="logInst" :line-height="1.5" :log="log" :font-size="10" />
    </n-card>
</template>

<style scoped>
.analyse-title {
    font-size: x-large;
    font-family: xiaoli;
}

.textlog {
    width: 260px;
    height: 440px;
    left: 400px;
    top: 0px;
    font-family: qiti;
}


</style>