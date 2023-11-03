<template>
  <div>
    <el-text class="mx-1">What is fast copy tool?</el-text>
    <el-button
      type="primary"
      text
      @click="gotoIntroduction"
      :disabled="taskStatus.isRunning"
      >&gt;&gt;&gt; View Introduction &lt;&lt;&lt;</el-button
    >
  </div>

  <el-divider />

  <el-row>
    <el-col :span="6">
      <el-statistic title="OS Name" :value="systemInfo.os" />
    </el-col>

    <el-col :span="6">
      <el-statistic title="CPU Cores" :value="systemInfo.cpuCores" />
    </el-col>

    <el-col :span="6">
      <el-statistic
        :value="(systemInfo.cpuUsage / systemInfo.cpuCores).toFixed(2)"
      >
        <template #title>
          <div style="display: inline-flex; align-items: center">CPU Usage</div>
        </template>
        <template #suffix>%</template>
      </el-statistic>
    </el-col>

    <el-col :span="6">
      <el-statistic :value="(systemInfo.usedMemory / 1073741824.0).toFixed(2)">
        <template #title>
          <div style="display: inline-flex; align-items: center">
            Memory Usage
          </div>
        </template>
        <template #suffix
          >/
          {{ (systemInfo.totalMemory / 1073741824.0).toFixed(1) }} GB</template
        >
      </el-statistic>
    </el-col>
  </el-row>

  <el-divider />

  <el-row>
    <el-col :span="11">
      <el-card class="bi-col-box-card">
        <template #header>
          <div class="card-header">
            <span>Source</span>

            <el-button
              type="primary"
              @click="selectInputFiles"
              :disabled="taskStatus.isRunning"
              >Select Directories</el-button
            >
          </div>
        </template>

        <div style="display: flex; flex-direction: column">
          <el-switch
            v-model="taskConfig.isCopyMode"
            active-text="Copy Files"
            inactive-text="Move Files"
            :disabled="taskStatus.isRunning"
          />

          <el-switch
            v-model="taskConfig.isSameDriver"
            active-text="Same Driver"
            inactive-text="Different Driver"
            :disabled="taskStatus.isRunning"
          />

          <el-checkbox
            v-model="taskConfig.ignoreHiddenFiles"
            :disabled="taskStatus.isRunning"
            >Ignore Hidden Files</el-checkbox
          >

          <el-checkbox
            v-model="taskConfig.ignoreEmptyFolders"
            :disabled="taskStatus.isRunning"
            >Ignore Empty Folders</el-checkbox
          >
        </div>
      </el-card>
    </el-col>

    <el-col :span="11" :offset="2">
      <el-card class="bi-col-box-card">
        <template #header>
          <div class="card-header">
            <span>Target</span>

            <el-button
              type="primary"
              @click="selectOutputFolder"
              :disabled="taskStatus.isRunning"
              >Select Directory</el-button
            >
          </div>
        </template>

        <div>
          <el-space direction="vertical" alignment="start" :size="20">
            <el-text>Target: {{ taskConfig.outputFolder }}</el-text>

            <el-text type="info"
              >All the selected input files / directories will be moved to this
              target directory.</el-text
            >
          </el-space>
        </div>
      </el-card>
    </el-col>
  </el-row>

  <el-card class="box-card" v-if="this.taskConfig.inputFiles.length > 0">
    <template #header>
      <div class="card-header">
        <span>Preview actions:</span>
        <el-button
          type="success"
          :loading="taskStatus.isRunning"
          :disabled="
            this.taskConfig.inputFiles.length <= 0 ||
            this.taskConfig.outputFolder.length <= 0
          "
          @click="startTask"
          >Confirm Task</el-button
        >
      </div>
    </template>

    <el-space direction="vertical" alignment="start">
      <el-text
        v-for="item in taskConfig.actionsPreview"
        :key="item"
        style="white-space: pre"
      >
        {{ item }}</el-text
      >
    </el-space>
  </el-card>

  <div
    v-if="taskStatus.isRunning || taskStatus.progress === 4"
    style="margin-top: 30px"
  >
    <el-steps
      :active="taskStatus.progress - 1"
      align-center
      finish-status="success"
    >
      <el-step title="Enumerating Files" description="Listing files..." />

      <el-step title="Preparing" description="Preparing sources..." />

      <el-step title="Processing" description="Processing task..." />

      <el-step title="Finished" description="Conguratulations!" />
    </el-steps>

    <el-progress
      :percentage="100"
      :status="colorIndicator(taskStatus.progress)"
      :indeterminate="taskStatus.progress > 0 && taskStatus.progress < 4"
      :duration="2"
      style="margin-top: 20px"
      :show-text="false"
    />

    <el-input
      v-model="taskStatus.messagesDisplay"
      :rows="15"
      type="textarea"
      placeholder="Empty Messages"
      disabled
      style="margin-top: 20px"
    />
  </div>
</template>

<script lang="ts">
import { getSystemInfo, getReadSpeed, getWriteSpeed } from "../ipc/system";
import { open, message } from "@tauri-apps/api/dialog";
import { resolve, join, basename } from "@tauri-apps/api/path";

export default {
  name: "HomePage",
  data() {
    return {
      systemInfoUpdaterId: void 0,
      systemInfo: {
        os: "",
        cpuCores: 1,
        cpuUsage: 1.0,
        totalMemory: 1073741824,
        usedMemory: 0,
      },
      taskConfig: {
        isCopyMode: true,
        ignoreHiddenFiles: false,
        ignoreEmptyFolders: true,
        isSameDriver: true,
        inputFiles: [],
        outputFolder: "",
        actionsPreview: [],
      },
      taskStatus: {
        isRunning: false,
        progress: 0,
        messages: [],
        messagesDisplay: "",
      },
    };
  },
  methods: {
    gotoIntroduction() {
      this.$router.push({ path: "/introduction" });
    },
    colorIndicator(progress: number): string {
      switch (progress) {
        default:
          return "info";
        case 1:
          return "danger";
        case 2:
          return "warning";
        case 3:
          return "primary";
        case 4:
          return "success";
      }
    },
    async updateSystemInfo(firstTime: boolean) {
      const system_info = await getSystemInfo();

      if (firstTime) {
        // 这三样是不可能在运行时发生变化的，所以只需要在第一次更新时获取即可，后面的直接缓存即可
        this.systemInfo.os = system_info.os;
        this.systemInfo.cpuCores = system_info.cpu_cores;
        this.systemInfo.totalMemory = system_info.total_memory;
      }

      this.systemInfo.cpuUsage = system_info.cpu_usage;
      this.systemInfo.usedMemory = system_info.used_memory;
    },
    async selectInputFiles() {
      const selected = await open({
        directory: true,
        multiple: true,
        recursive: false,
        title: "Select Input Directories",
      });

      if (Array.isArray(selected)) {
        this.taskConfig.inputFiles = selected;
      } else if (!selected) {
        // 啥都没有选
        this.taskConfig.inputFiles = [];
      } else {
        // select single file
        this.taskConfig.inputFiles = [selected];
      }

      if (!(await this.judgeDirectory())) {
        this.taskConfig.outputFolder = "";
        await message("Target directory can't be inside source directory!", {
          title: "File Error",
          type: "error",
        });
      }

      await this.makeTaskPreview();
    },
    async selectOutputFolder() {
      const selected = await open({
        directory: true,
        multiple: false,
        recursive: false,
        title: "Select Output Folder",
      });

      this.taskConfig.outputFolder = selected ?? "";

      if (!(await this.judgeDirectory())) {
        this.taskConfig.outputFolder = "";
        await message("Target directory can't be inside source directory!", {
          title: "File Error",
          type: "error",
        });
      }

      await this.makeTaskPreview();
    },
    async judgeDirectory(): Promise<boolean> {
      if (
        this.taskConfig.inputFiles.length <= 0 ||
        this.taskConfig.outputFolder.length <= 0
      ) {
        return true;
      }

      for (let input of this.taskConfig.inputFiles) {
        // 目标文件夹在源文件夹内，不允许
        if (this.taskConfig.outputFolder.startsWith(input)) return false;

        // 目标文件夹是源文件夹的父目录，不允许
        const parentDir = await resolve(input, "..");
        const targetDir = await resolve(this.taskConfig.outputFolder, ".");

        if (parentDir === targetDir) return false;
      }

      return true;
    },
    async makeTaskPreview() {
      if (this.taskConfig.inputFiles.length <= 0) {
        this.taskConfig.actionsPreview = [];
        return;
      }

      const tasks = this.taskConfig.inputFiles.map(
        async (f: string): Promise<string> => {
          const filename = await basename(f);

          if (this.taskConfig.outputFolder.length <= 0) {
            return `${f}   ->   PLEASE_SPECIFY_TARGET_FOLDER`;
          }

          const newFileName = await join(
            this.taskConfig.outputFolder,
            filename
          );

          return `${f}   ->   ${newFileName}`;
        }
      );

      this.taskConfig.actionsPreview = await Promise.all(tasks);
    },
    async startTask() {
      this.taskStatus.isRunning = true;
      this.taskStatus.messages = [];

      this.taskStatus.progress = 1;
      this.taskStatus.messages.push("Enumerating files ...");
      this.taskStatus.messages.push(
        `Selected ${this.taskConfig.inputFiles.length} sources, expanding sub folders and files ...`
      );
      this.buildLogDisplay();

      this.taskStatus.progress = 2;
      this.taskStatus.messages.push("Preparing sources ...");
      this.taskStatus.messages.push(
        "Doing speed test and collecting information for algorithm ..."
      );
      this.buildLogDisplay();

      const readBytesPerSecond = await getReadSpeed(
        this.taskConfig.inputFiles[0]
      );
      this.taskStatus.messages.push(
        `Read speed: ${(readBytesPerSecond / 1048576).toFixed(3)} MB/s.`
      );
      this.taskStatus.messages.push(
        `It seems that the source is a ${
          readBytesPerSecond > 5242880 ? "SSD" : "HDD"
        }`
      );
      this.buildLogDisplay();

      const writeBytesPerSecond = await getWriteSpeed(
        this.taskConfig.outputFolder
      );
      this.taskStatus.messages.push(
        `Write speed: ${(writeBytesPerSecond / 1048576).toFixed(3)} MB/s.`
      );
      this.taskStatus.messages.push(
        `It seems that the target is a ${
          writeBytesPerSecond > 5242880 ? "SSD" : "HDD"
        }`
      );
      this.buildLogDisplay();

      this.taskStatus.progress = 3;
      this.taskStatus.messages.push("Processing ...");
      this.buildLogDisplay();
    },
    buildLogDisplay() {
      this.taskStatus.messagesDisplay = this.taskStatus.messages.join("\n");
    },
  },
  async mounted() {
    await this.updateSystemInfo(true);
    this.systemInfoUpdaterId = setInterval(() => {
      this.updateSystemInfo(false);
    }, 1000);
  },
  unmounted() {
    clearInterval(this.systemInfoUpdaterId);
  },
};
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.bi-col-box-card {
  height: 225px;
}

.box-card {
  margin-top: 25px;
}
</style>