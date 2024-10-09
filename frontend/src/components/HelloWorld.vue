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
const input_value = ref('')
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


const addNewEnv = ()=>{
  ElMessageBox.prompt('请输入名称(key):', '新建环境变量', {
    confirmButtonText: '确认',
    cancelButtonText: '取消',
  })
    .then(({ value }) => {
      invoke('add_new_env', {key: value})
      .then((flag) => {
        if(flag === 0){
          ElMessage({
            type: 'success',
            message: `成功添加环境变量: ${value}`,
          })
          getAllEnv()
        }else if(flag === 1) {
          ElMessage({
            type: 'error',
            message: '内部错误,请联系管理员',
          })
        }else if(flag === 2){
          ElMessage({
            type: 'warning',
            message: `环境变量已存在!: ${value}`,
          })
        }
      })
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '取消输入',
      })
    })
}

const delEnv = ()=>{
  if(active_value.value === ''){
    ElMessage({
      type: 'error',
      message: '未选择任何环境变量!',
    })
    return false
  }
  invoke('del_env', {key: active_value.value})
      .then((flag) => {
        if(flag === 0){
          ElMessage({
            type: 'success',
            message: `成功删除环境变量: ${active_value.value}`,
          })
          getAllEnv()
        }else if(flag === 1) {
          ElMessage({
            type: 'error',
            message: '内部错误,请联系管理员',
          })
        }
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
      <el-button type="danger" @click="delEnv" text bg>删除</el-button>
      <el-button type="success" @click="updateEnv" text bg>保存</el-button>
    </el-col>
  </el-row>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
