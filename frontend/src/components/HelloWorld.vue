<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

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
</script>

<template>
  <el-row :gutter="12">
    <el-col :span="24">
      <el-select 
      v-model="active_value" 
      class="m-2" 
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
    <el-col :span="24">
      <el-input
        v-model="active_value"
        :rows="2"
        type="textarea"
        placeholder="Please input"
      />
    </el-col>
  </el-row>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
