
<script lang="ts">
import { invoke } from "@tauri-apps/api";

export default {
  data() {
    return {
      profiles: [
        { name: "Profile 1", value: "profile_1"},
        { name: "Profile 2", value: "profile_2"},
        { name: "Profile 3", value: "profile_3"},
      ],
      selectedProfile: "",
    }
  },
  created() {
    this.selectedProfile = this.profiles[0].value;
  },
  methods: {
    async open_dialog() {
      console.log("Initialized dialog");
      await invoke("open_message_dialog", {
        message: this.selectedProfile
      });
    }
  }
}

</script>

<template>
    <div class="downbar">
        <div class="profile-picker">
          <div class="profile-section">
            <span>Profile:</span>
            <select v-model="selectedProfile">
                <option v-for="profile in profiles" :value="profile.value">{{ profile.name }}</option>
            </select>
          </div>
          <div>
            <button class="options-btn">Options</button>
          </div>
        </div>
        <div class="play">
            <button @click="open_dialog" class="play-btn">Play</button>
        </div>
        <div class="user-info">
            User Info
        </div>
    </div>
</template>

<style scoped>
* {
  display: grid;
}
.downbar {
  background-color: #303030;
  position: fixed;
  margin: 0;
  padding: 10px 10px;
  left: 0;
  right: 0;
  width: 100% !important;
  bottom: 0;
  grid-template-columns: 20% 60% 20%;
  align-items: stretch;
  justify-content: space-between;
}
.profile-picker {
  align-items: center;
  justify-items: stretch;
  grid-template-rows: 50% 50%;
}
.profile-section {
  padding: 5px;
  width: 100%;
  justify-content: space-between;
  grid-template-columns: auto auto;
}
.options-btn {
  width: 100%;
}
.play {
  justify-items: center;
}
.play-btn {
  align-items: center;
  padding: 15px 15vw;
  text-transform: uppercase;
  font-weight: 600;
  font-family: "Roboto Thin", sans-serif;
  font-size: 20px;
  border-radius: 5px;
  background-color: #05e205;
}
.user-info {
  align-items: center;
  justify-content: center;
}
</style>