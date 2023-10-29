<script setup lang="ts">
import { useDark } from "@vueuse/core";

const isDark = useDark();
</script>

<template>
  <el-container>
    <el-header>
      <el-row>
        <el-col :span="20">
          <h1>Fast Copy Tool</h1>
        </el-col>

        <el-col :span="4">
          <el-switch
            v-model="isDark"
            class="mb-2"
            active-text="Dark"
            inactive-text="Light"
          />
        </el-col>
      </el-row>
    </el-header>

    <el-main>
      <router-view />
    </el-main>
  </el-container>
</template>

<style scoped>
</style>
