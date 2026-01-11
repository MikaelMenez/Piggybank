<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const usuario = ref('')
const senha = ref('')

function fazerLogin() {
  // 1. Verifica Usuário
  if (!usuario.value) {
    alert("Por favor, digite um usuário!")
    return
  }

  // 2. Verifica Senha (NOVO)
  if (!senha.value) {
    alert("Por favor, digite a senha!")
    return
  }

  // 3. Sucesso
  localStorage.setItem("userName", usuario.value)
  alert("Login realizado com sucesso! Bem-vindo, " + usuario.value)
  router.push('/dashboard')
}
</script>

<template>
  <div class="login-page">
    <div class="container">
      <h1>Acesse sua conta</h1>
      
      <div id="usuario">
        <label for="name-usuario">Seu Usuário:</label>
        <input 
          type="text" 
          id="name-usuario" 
          v-model="usuario" 
          placeholder="Digite aqui o seu usuário"
        >
      </div>

      <div id="senha">
        <label for="senha-usuario">Sua Senha:</label>
        <input 
          type="password" 
          id="senha-usuario" 
          v-model="senha"
          placeholder="Digite sua senha aqui"
        >      
      </div>

      <div>
        <button id="botao-login" @click="fazerLogin">ACESSAR CONTA</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
  /* --- AQUI ESTAVA O PROBLEMA: TROCAMOS 'body' POR '.login-page' --- */
.login-page 
{
    font-family: Arial, sans-serif;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    margin: 0;
    background-color: #68473d; /* Aquele marrom */
    width: 100%;
}

.container 
{
    text-align: center;
    background-color: #998267;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    width: 90%;
    max-width: 600px;
    border: 2px solid #8B7D70;
}

/* Inputs */
input 
{
    width: 300px;
    padding: 10px;
    margin-top: 5px;
    border-radius: 10px;
    border: 1px solid #ccc;
    font-size: 16px;
    outline: none;
}

input:focus 
{
    border-color: #FFAAEA;
}

label 
{
    display: block;
    color: white;
    font-size: 18px;
    font-weight: bold;
    font-family: Arial, sans-serif;
    margin-bottom: 8px;
    margin-left: 5px;
    margin-top: 20px;
    text-shadow: 1px 1px 2px black;
}

#botao-login {
    width: 100%;
    max-width: 300px;
    padding: 12px 20px;
    background: linear-gradient(45deg, #ff85d8, #FFAAEA);
    color: white;
    border: none;
    font-size: 16px;
    font-weight: bold;
    text-transform: uppercase;
    letter-spacing: 1px;
    border-radius: 50px;
    box-shadow: 0 4px 15px rgba(255, 105, 180, 0.4);
    cursor: pointer;
    transition: all 0.3s ease;
    margin-top: 30px;
}

#botao-login:hover {
    background: linear-gradient(45deg, #ff69c9, #ff94dd);
    transform: translateY(-3px);
    box-shadow: 0 6px 20px rgba(255, 105, 180, 0.6);
}

#botao-login:active {
    transform: translateY(1px);
    box-shadow: 0 2px 10px rgba(255, 105, 180, 0.4);
}
h1
{
    font-size: 30px;
    font-weight: 700;
    color: rgb(0, 0, 0);
}
</style>