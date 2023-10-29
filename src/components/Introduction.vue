<template>
  <el-page-header @back="goBack">
    <template #content>
      <el-text class="mx-1" type="primary" size="large">Introduction</el-text>
    </template>
  </el-page-header>

  <el-space direction="vertical" alignment="start">
    <el-text class="mx-1" type="info"
      >快速拷贝工具是一个基于现代操作系统与现代硬件的文件复制（移动）工具，它不同于系统自带的文件管理器，本工具会充分利用现代操作系统的大内存、多线程能力以及SSD的读写能力，从而提高文件复制（移动）的效率。</el-text
    >

    <el-text size="large" class="mx-1" type="danger">适用条件：</el-text>

    <el-text class="mx-1" type="info"
      >在不恰当的场景下使用本软件，可能并不能获得想要的提速效果。快速拷贝工具依然受到机器物理性能的限制，如果源设备与目标设备的性能本身就较差的情况下，使用本工具并不能产生质的飞跃。</el-text
    >

    <el-text class="mx-1"
      >1，拥有<el-text class="mx-1" type="danger">8GB</el-text
      >以上的内存。否则产生交换内存后，拷贝的效率将会急剧降低。</el-text
    >

    <el-text class="mx-1"
      >2，拷贝的文件中<el-text class="mx-1" type="danger"
        >含有大量小文件</el-text
      >。由于磁盘的特性，拷贝大文件时，快速拷贝工具的效率并不会更高，只会对小文件有较好的加速效果。</el-text
    >

    <el-text class="mx-1"
      >3，目标磁盘请尽量使用<el-text class="mx-1" type="danger">SSD</el-text
      >。由于机械硬盘的特性，其读写性能本来就较为低下，因此使用快速拷贝工具提升的效率有限，直接使用系统的文件管理器也并不会慢多少。</el-text
    >

    <el-text class="mx-1"
      >4，源磁盘为机械硬盘时，建议先进行<el-text class="mx-1" type="danger"
        >磁盘碎片整理</el-text
      >。由于机械硬盘的特性，随机寻道会导致读取性能急剧下降，因此建议先进行磁盘整理，快速拷贝工具将会对机械硬盘的连续读取会有优化，可以极大提升机械硬盘读取小文件的效率。</el-text
    >

    <el-text class="mx-1" type="warning" size="large">软件特性：</el-text>

    <el-text class="mx-1" type="info"
      >本软件追求的是极致效率，因此在一些方面有取舍，例如文件严格的完整性等等，详细的软件特性将会在下方说明。</el-text
    >

    <el-text class="mx-1"
      >1，文件拷贝（移动）时，<el-text class="mx-1" type="danger"
        >不会进行文件的完整性校验</el-text
      >。文件的完整性校验是一个非常耗时的工作，校验文件的完整性消耗的时间可能是复制（移动）文件本身所耗时间的数倍，因此软件只会比对文件名以及文件大小相同后，就判断操作成功。</el-text
    >

    <el-text class="mx-1"
      >2，文件拷贝（移动）时，<el-text class="mx-1" type="danger"
        >可能会丢失文件的元信息</el-text
      >。例如修改时间、作者、文件描述等等，但不会影响文件本身的内容。</el-text
    >

    <el-text class="mx-1"
      >3，在操作开始时，软件会自动检测一次源磁盘与目标磁盘的读写速度，然后根据速度再自动选择最佳的操作方式。因此，您可能看到软件刚启动操作时没有在做文件拷贝（移动）操作，这是正常现象。</el-text
    >

    <el-text class="mx-1"
      >4，本软件可以拷贝（移动）到移动硬盘、U盘、网络位置等任何形式的设备上，但是本软件自身不实现任何的文件服务协议。您先要将源磁盘和目标挂载到当前的计算机上，才能进行拷贝（移动）操作。</el-text
    >

    <el-text class="mx-1"
      >5，本软件会自动判断之前处理过的文件，并自动从断点开始处理，因此如果您想继续操作时，直接选择源磁盘与目标磁盘即可，软件将会自动找到两边相差的文件，并进行补全。</el-text
    >

    <el-text class="mx-1"
      >6，本软件在进行文件操作时，会<el-text class="mx-1" type="danger"
        >尽可能榨取系统的极限性能</el-text
      >，因此会将IO、CPU以及内存占用都拉高到一个非常高的水平。因此，请您不要在进行文件拷贝（移动）的时候让计算机进行其它繁重的工作，以免影响到您的使用体验。</el-text
    >
  </el-space>
</template>

<script lang="ts">
export default {
  name: "Introduction",
  methods: {
    goBack() {
      this.$router.back();
    },
  },
};
</script>

<style>
</style>