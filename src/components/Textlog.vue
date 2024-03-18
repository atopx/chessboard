<script setup lang="ts">
import { LogInst, NCard, NLog } from 'naive-ui';
import { nextTick, onMounted, ref, watchEffect } from 'vue';


const log=`
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

</script>

<template>
  <n-card title="局面分析"
  :bordered="false"
  class="textlog"
  content-style="color: blue"
  >
    <n-log :rows=16 ref="logInst" :line-height="1.5" :log="log" :font-size="10"/>
  </n-card>
</template>

<style scoped>

.textlog {
  width: 260px;
  left: 400px;
  top: 20px;
  font-family: qiti;
}
</style>