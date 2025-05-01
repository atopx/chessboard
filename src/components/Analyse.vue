<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { LogInst, NCard, NDivider, NFlex, NLog, NText } from 'naive-ui';
import { ref } from 'vue';

interface Analyse {
    depth: number,   // 深度
    score: number,   // 得分
    time: number,    // 时间
    pvs: string[],   // 思考(iccs)
    moves: string[], // 思考(chinese)
    state: string,   // 状态
    source: string,  // 来源
}


const logs = ref([""])
const best = ref({
    move: "---",
    depth: 0,
    score: 0,
})

listen('analyse', async (event) => {
    let data = event.payload as Analyse;
    let mvs = data.moves.join(" ");
    logs.value.push(`<${data.source}> ${mvs}`)
    // 滚动到最新
    if (logs.value.length > 18) {
        if (logs.value.length > 128) {
            // 清理很早以前的数据
            logs.value.shift();
        }
        logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
    }
    best.value.move = data.moves[0];
    best.value.depth = data.depth;
    best.value.score = data.score;

    // 设置b-select
    let pv = data.pvs[0];
    let from = pv.substring(0, 2);
    let to = pv.substring(2, 4)
    document.getElementById(from)?.classList.add("b-select");
    document.getElementById(to)?.classList.add("b-select");
})


const logInstRef = ref<LogInst | null>(null)

</script>

<template>
    <n-card title="局面分析" :bordered="false" class="textlog" content-style="color: blue">
        <n-flex justify="space-between" align="end">
            <n-text type="info" class="analyse-title" strong>
                {{ best.move }}
            </n-text>

            <n-text type="error">
                {{ best.score }}
            </n-text>
            <n-text type="warning">
                {{ best.depth }}
            </n-text>
        </n-flex>
        <n-divider />
        <n-log class="analyse-log" :rows=18 ref="logInst" :line-height="1.5" :lines="logs" :font-size="10" />
    </n-card>
</template>

<style scoped>
.analyse-title {
    font-size: x-large;
}

.textlog {
    width: 260px;
    height: 440px;
    left: 400px;
    top: 0px;
}
</style>