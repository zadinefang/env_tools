<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { ElMessage, ElMessageBox } from 'element-plus'

defineProps<{ msg: string }>()

interface Option {
  value: string;
  label: string;
}

const active_value = ref('')
const options= ref<Option[]>([])

onMounted(()=>{
  getAllEnv()
})

const getAllEnv = () => {
  invoke('get_all_env', {})
  // `invoke` 返回的是一个 Promise
  .then((values) => {
    let options_arr = []
    for (const val of values as Array<String>) {
      options_arr.push({
        value: val[1],
        label: val[0]
      })
    }
    options.value = options_arr
  })
}

const decrypt = () => {
  if(active_value.value === ''){
    ElMessage({
      type: 'error',
      message: '未选择任何环境变量!',
    })
    return false
  }

  invoke('decrypt', {input: active_value.value, salt: 'itsp'})
      .then((resText) => {
        const result = resText as string
        ElMessageBox.confirm(result, '结果', {
          confirmButtonText: '复制',
          cancelButtonText: '关闭',
        }).then(async () => {
          await navigator.clipboard.writeText(result)
        })
      })
}


const cryption = ()=>{
  ElMessageBox.prompt('请输入加密密钥:', '加密', {
    confirmButtonText: '确认',
    cancelButtonText: '取消',
  })
    .then(({ value }) => {
      invoke('encrypt', {input: value, salt: "itsp"})
      .then((resText) => {
        const result = resText as string
        ElMessageBox.confirm(result, '结果', {
          confirmButtonText: '复制',
          cancelButtonText: '关闭',
        }).then(async () => {
          await navigator.clipboard.writeText(result)
        })
      })
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '取消输入',
      })
    })
}

</script>

<template>
  <el-row :gutter="12">
    <el-col :span="24">
      <el-select 
      v-model="active_value" 
      filterable 
      placeholder="Select" 
      size="large"
      >
        <el-option
          v-for="item in options"
          :key="item.value"
          :label="item.label"
          :value="item.value"
        />
      </el-select>
    </el-col>
    <el-col :span="24" class="mt-2">
      <el-input
        v-model="active_value"
        :rows="6"
        type="textarea"
        placeholder="Please input"
      />
    </el-col>
    <el-col :span="24" class="mt-2">
      <el-button type="primary" @click="cryption" text bg>加密</el-button>
      <el-button type="primary" @click="decrypt" text bg>解密</el-button>
    </el-col>
  </el-row>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
