<script setup>
import { RouterLink } from 'vue-router'
</script>

<template>
  <div class="home-body">
    <header>
      <div id="upper">
        <div id="logo">
          <img src="/images/Piggy - Copia.png" alt="LogoPiggy">
          <p>PiggyBank</p>
        </div>
        <div id="upperbtns">
          <RouterLink to="/login" style="text-decoration: none;">
            <button>Acesse sua conta</button>
          </RouterLink>
        </div>
      </div>
    </header>
    <main>
      <div id="glazing">
        <h1>O melhor gerenciador financeiro para situações do dia a dia</h1>
      </div>
    </main>
  </div>
</template>

<style scoped>
/* CSS Trazido do index.html */
.home-body {
  font-family: Arial, Helvetica, sans-serif;
  background-image: url('/images/porquinho.jpg'); /* Caminho ajustado para public */
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  background-attachment: fixed;
  min-height: 100vh;
  margin: 0;
}

#upper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #d3d3d3;
  height: 56px;
  border: none;
  box-shadow: 0 4px 12.1px rgba(0, 0, 0, 0.5);
}

#logo {
  display: flex;
  justify-content: left;
  align-items: center;
  width: 177px;
  height: 32px;
  margin-left: 20px;
  gap: 10px;
}

#logo p {
  font-size: 24px;
  margin-top: 0px;
  color: black;
}

#upperbtns {
  display: flex;
  gap: 43px;
  margin-right: 30px;
}

img {
  width: 32px;
  height: 32px;
}

button {
  height: 29px;
  width: 172px;
  margin: 10px;
  font-size: 16px;
  border-radius: 10px;
  border: none;
  background: linear-gradient(45deg, #ff85d8, #FFAAEA);
  color: white;
  font-weight: bold;
  text-shadow: 0 1px 2px rgba(0,0,0,0.2);
  box-shadow: 0 4px 4px rgba(0, 0, 0, 0.25);
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  justify-content: center;
  align-items: center;
}

button:hover {
  background: linear-gradient(45deg, #ff69c9, #ff94dd);
  transform: translateY(-3px);
  box-shadow: 0 6px 8px rgba(0, 0, 0, 0.3);
}

button:active {
  transform: translateY(1px);
  box-shadow: 0 2px 3px rgba(0, 0, 0, 0.2);
}

h1 {
  font-size: 48px;
  font-weight: 500;
  color: white;
}

#glazing {
  width: 600px;
  margin-left: 20px;
}

main {
  display: flex;
  justify-content: left;
  align-items: center;
  height: calc(100vh - 56px);
}
</style>