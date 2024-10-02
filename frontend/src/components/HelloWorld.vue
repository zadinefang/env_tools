<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { ElMessage, ElMessageBox } from 'element-plus'

defineProps<{ msg: string }>()

const active_value = ref('')
const options = ref([])

invoke('get_all_env', {})
  // `invoke` 返回的是一个 Promise
  .then((values) => {
    let options_arr = []
    for (const val of values) {
      options_arr.push({
        value: val[1],
        label: val[0]
      })
    }
    options.value = options_arr
  })

const addNewEnv = ()=>{
  ElMessageBox.prompt('请输入名称(key):', '新建环境变量', {
    confirmButtonText: '确认',
    cancelButtonText: '取消',
  })
    .then(({ value }) => {
      ElMessage({
        type: 'success',
        message: `您输入的是: ${value}`,
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
      <el-button type="primary" @click="addNewEnv" text bg>新增</el-button>
      <el-button type="danger" text bg>删除</el-button>
      <el-button type="success" text bg>保存</el-button>
    </el-col>
  </el-row>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
