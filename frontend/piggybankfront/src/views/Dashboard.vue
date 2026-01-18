<script setup>
import { ref, onMounted, computed, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import Chart from 'chart.js/auto'

const router = useRouter()
const API_URL = "http://127.0.0.1:46000"

// --- ESTADO ---
const nomeUsuario = ref('Visitante')
const saldoTotal = ref(0)
const listaGlobalTransacoes = ref([])
const dataAtual = ref(new Date())

// Modais
const modalAberto = ref(false) // Modal de Nova Transação
const modalGraficoAberto = ref(false) // Modal do Gráfico (NOVO)

const tipoPersonalizadoVisivel = ref(false)
const form = ref({
  valor: '',
  tipo: 'entrada',
  customTipo: ''
})

// Filtros
const filtroAtivo = ref('tudo')

// Gráfico
const graficoCanvas = ref(null)
let graficoInstance = null

// --- COMPUTADOS ---
const nomeMesAno = computed(() => {
  return dataAtual.value.toLocaleDateString('pt-BR', { month: 'long', year: 'numeric' }).toUpperCase()
})

const saldoFormatado = computed(() => {
  return saldoTotal.value.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' })
})

// A última transação sempre será a primeira na lista
const transacoesFiltradas = computed(() => {
  let lista = listaGlobalTransacoes.value

  if (filtroAtivo.value === 'entradas') {
    lista = lista.filter(t => t.valor > 0)
  } else if (filtroAtivo.value === 'saídas') {
    lista = lista.filter(t => t.valor < 0)
  }

  // AQUI: Transforma texto em data e ordena da mais nova pra mais velha
  return [...lista].sort((a, b) => new Date(b.data) - new Date(a.data))
})

// --- MÉTODOS ---

onMounted(() => {
  const user = localStorage.getItem("userName")
  if (!user) {
    router.push('/login')
  } else {
    nomeUsuario.value = user
    carregarTransacoes()
  }
})

function mudarMes(delta) {
  const novaData = new Date(dataAtual.value)
  novaData.setMonth(novaData.getMonth() + delta)
  dataAtual.value = novaData
  carregarTransacoes()
}

async function carregarTransacoes() {
  try {
    const mes = dataAtual.value.getMonth() + 1
    const ano = dataAtual.value.getFullYear()

    const res = await fetch(`${API_URL}/transactions/by_date/${mes}/${ano}`)

    if (res.ok) {
      listaGlobalTransacoes.value = await res.json()
    } else if (res.status === 404) {
      listaGlobalTransacoes.value = []
    }

    calcularSaldo()
    // Nota: Não desenhamos o gráfico aqui automaticamente mais,
    // pois o canvas pode estar escondido (modal fechado).

  } catch (error) {
    console.error("Erro API:", error)
    listaGlobalTransacoes.value = []
    calcularSaldo()
  }
}

function calcularSaldo() {
  saldoTotal.value = listaGlobalTransacoes.value.reduce((acc, item) => acc + item.valor, 0)
}

async function salvarTransacao() {
  let tipoFinal = form.value.tipo
  if (tipoFinal === 'outro') {
    tipoFinal = form.value.customTipo.trim()
    if (!tipoFinal) return alert("Digite o tipo personalizado!")
  }

  let valorFinal = parseFloat(form.value.valor)
  if (tipoFinal.toLowerCase() !== 'entrada') {
    valorFinal = valorFinal * -1
  }

  const payload = { tipo: tipoFinal, valor: valorFinal }

  try {
    const res = await fetch(`${API_URL}/add_transaction`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    })

    if (res.ok) {
      modalAberto.value = false
      form.value.valor = ''
      form.value.tipo = 'entrada'
      form.value.customTipo = ''
      tipoPersonalizadoVisivel.value = false
      carregarTransacoes()
    } else {
      alert("Erro ao salvar")
    }
  } catch (e) {
    alert("Erro de conexão")
  }
}

async function excluir(id) {
  if (!confirm("Excluir item?")) return
  try {
    const res = await fetch(`${API_URL}/delete_transaction/${id}`, { method: 'DELETE' })
    if (res.ok) carregarTransacoes()
  } catch (e) {
    alert("Erro ao excluir")
  }
}

// --- LÓGICA DO GRÁFICO (Alterada) ---
async function abrirGrafico() {
    modalGraficoAberto.value = true;
    // nextTick espera o Vue renderizar o HTML do modal para o <canvas> existir
    await nextTick();
    desenharGrafico();
}

function desenharGrafico() {
  if (!graficoCanvas.value) return

  const lista = listaGlobalTransacoes.value
  const entradas = lista.filter(t => t.valor > 0).reduce((acc, t) => acc + t.valor, 0)
  const saidas = lista.filter(t => t.valor < 0).reduce((acc, t) => acc + (t.valor * -1), 0)

  if (graficoInstance) graficoInstance.destroy()

  // Se não tiver dados, podemos desenhar um gráfico vazio ou retornar
  // Vou deixar desenhar para mostrar zerado

  graficoInstance = new Chart(graficoCanvas.value, {
    type: 'doughnut',
    data: {
      labels: ['Entradas', 'Saídas'],
      datasets: [{
        data: [entradas, saidas],
        backgroundColor: ['#10b981', '#ef4444'],
        borderWidth: 0,
        hoverOffset: 4
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { position: 'bottom' } }
    }
  })
}

function logout() {
  localStorage.removeItem("userName")
  router.push('/login')
}

function verificarTipoCustom() {
  tipoPersonalizadoVisivel.value = (form.value.tipo === 'outro')
}

const formatarData = (dataStr) => new Date(dataStr).toLocaleDateString('pt-BR', { day: '2-digit', month: '2-digit' })
</script>

<template>
  <div class="dashboard-body">
    <header>
        <div id="upper">
            <div id="logo">
                <img src="/images/Piggy - Copia.png" alt="LogoPiggy">
                <p id="Piggy">PiggyBank</p>
            </div>
            <div id="saudation">
                <p>Saudações, <span>{{ nomeUsuario }}</span>!</p>
                <button id="btnSair" @click="logout">Sair</button>
            </div>
        </div>
    </header>

    <main class="dashboard-container">
        <section id="area-saldo">
            <p>Saldo Total</p>
            <h1>{{ saldoFormatado }}</h1>

            <button class="btn-ver-grafico" @click="abrirGrafico">
                📊 Ver Gráfico
            </button>
        </section>

        <div class="controles-topo">
            <div class="seletor-data">
                <button @click="mudarMes(-1)">&lt;</button>
                <span id="data-atual">{{ nomeMesAno }}</span>
                <button @click="mudarMes(1)">&gt;</button>
            </div>
            <button id="btn-nova-transacao" @click="modalAberto = true">+ NOVA</button>
        </div>

        <div class="filtros">
            <button class="filtro-btn" :class="{ ativo: filtroAtivo === 'tudo' }" @click="filtroAtivo = 'tudo'">Tudo</button>
            <button class="filtro-btn" :class="{ ativo: filtroAtivo === 'entradas' }" @click="filtroAtivo = 'entradas'">Entradas</button>
            <button class="filtro-btn" :class="{ ativo: filtroAtivo === 'saídas' }" @click="filtroAtivo = 'saídas'">Saídas</button>
        </div>

        <ul id="lista-transacoes">
            <li v-for="item in transacoesFiltradas" :key="item.id" class="item-transacao">
                <div class="info-esquerda">
                    <div class="icone-tipo" :class="item.valor > 0 ? 'entrada' : 'saida'">
                        {{ item.valor > 0 ? '⬇' : '⬆' }}
                    </div>
                    <div class="detalhes">
                        <strong class="titulo">{{ item.tipo.toUpperCase() }}</strong>
                        <span class="data">{{ " "+formatarData(item.data) }}</span>
                    </div>
                </div>
                <div class="info-direita">
                    <strong class="valor" :class="item.valor > 0 ? 'positivo' : 'negativo'">
                        {{ item.valor.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' }) }}
                    </strong>
                    <div class="acoes">
                        <button class="btn-excluir" @click="excluir(item.id)">🗑️</button>
                    </div>
                </div>
            </li>
        </ul>
    </main>

    <div id="modal-overlay" v-if="modalAberto" @click.self="modalAberto = false">
        <div class="modal-box">
            <h2>Nova Transação</h2>
            <form @submit.prevent="salvarTransacao">
                <div class="campo">
                    <label>Valor (R$)</label>
                    <input type="number" step="0.01" required v-model="form.valor" placeholder="0,00">
                </div>
                <div class="campo">
                    <label>Tipo</label>
                    <select v-model="form.tipo" @change="verificarTipoCustom" required>
                        <option value="entrada">🟢 Entrada (Ganho)</option>
                        <option value="lazer">🔴 Lazer</option>
                        <option value="supermercado">🔴 Supermercado</option>
                        <option value="saida">🔴 Outra Saída</option>
                        <option value="outro">✨ Personalizado...</option>
                    </select>
                    <input v-if="tipoPersonalizadoVisivel" type="text" v-model="form.customTipo" placeholder="Digite o nome..." style="margin-top: 10px;">
                </div>
                <div class="botoes-modal">
                    <button type="button" id="btn-cancelar" @click="modalAberto = false">Cancelar</button>
                    <button type="submit" id="btn-salvar">Salvar</button>
                </div>
            </form>
        </div>
    </div>

    <div id="modal-overlay" v-if="modalGraficoAberto" @click.self="modalGraficoAberto = false">
        <div class="modal-box modal-grafico-box">
            <div class="modal-header">
                <h2>Resumo Mensal</h2>
                <button class="btn-fechar-x" @click="modalGraficoAberto = false">✕</button>
            </div>

            <div class="grafico-container-modal">
                <canvas ref="graficoCanvas"></canvas>
            </div>
        </div>
    </div>

  </div>
</template>

<style scoped>
/* (MANTIVE SEU CSS ANTIGO E ADICIONEI OS NOVOS ABAIXO) */

.dashboard-body {
    background-color: #11034b86;
    min-height: 100vh;
    width: 100%;
    font-family: Arial, sans-serif;
    display: flex;
    flex-direction: column;
}

header {
    width: 100%;
    background-color: #d3d3d3;
    box-shadow: 0 4px 12.1px rgba(0, 0, 0, 0.5);
    z-index: 10;
}

#upper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 56px;
    padding: 0 20px;
    max-width: 1200px;
    margin: 0 auto;
}

#logo {
    display: flex;
    align-items: center;
    gap: 10px;
}
#logo img { width: 32px; height: 32px; }
#Piggy { font-size: 24px; color: black; margin: 0; font-weight: bold; }

#saudation {
    display: flex;
    align-items: center;
    font-size: 18px;
    gap: 15px;
}

#btnSair {
    height: 35px;
    padding: 0 20px;
    background: linear-gradient(45deg, #ff85d8, #FFAAEA);
    color: white;
    border: none;
    border-radius: 10px;
    font-weight: bold;
    cursor: pointer;
    box-shadow: 0 2px 5px rgba(0,0,0,0.2);
}

.dashboard-container {
    width: 100%;
    max-width: 600px;
    margin: 20px auto;
    padding: 0 20px;
    flex: 1;
}

#area-saldo {
    text-align: center;
    background: linear-gradient(135deg, #FFAAEA, #ff85d8);
    padding: 30px;
    border-radius: 20px;
    color: white;
    box-shadow: 0 10px 20px rgba(255, 105, 180, 0.3);
    margin-bottom: 25px;
}
#area-saldo h1 { font-size: 42px; margin-top: 10px; }

/* Estilo novo do Botão Ver Gráfico */
.btn-ver-grafico {
    margin-top: 15px;
    background-color: rgba(255, 255, 255, 0.2);
    border: 1px solid white;
    color: rgb(0, 26, 255);
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    transition: 0.2s;
    font-weight: bold;
}
.btn-ver-grafico:hover {
    background-color: white;
    color: #ff85d8;
}

.controles-topo {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
.seletor-data {
    display: flex;
    align-items: center;
    gap: 10px;
    font-weight: bold;
    color: white;
}
.seletor-data button {
    width: 30px; height: 30px;
    border-radius: 50%;
    border: 1px solid #ccc;
    background: white;
    cursor: pointer;
}

#btn-nova-transacao {
    background-color: #333;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 20px;
    font-weight: bold;
    cursor: pointer;
}

.filtros {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}
.filtro-btn {
    background-color: white;
    border: 1px solid #eee;
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    color: #666;
}
.filtro-btn.ativo {
    background-color: #FFAAEA;
    color: white;
    border-color: #FFAAEA;
}

#lista-transacoes { list-style: none; padding: 0; }
.item-transacao {
    background-color: white;
    padding: 15px;
    margin-bottom: 12px;
    border-radius: 15px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 4px 10px rgba(0,0,0,0.03);
}

.info-esquerda { display: flex; align-items: center; gap: 15px; }
.icone-tipo {
    width: 40px; height: 40px; border-radius: 10px;
    display: flex; align-items: center; justify-content: center; font-size: 20px;
}
.icone-tipo.entrada { background-color: #d1fae5; color: #065f46; }
.icone-tipo.saida { background-color: #fee2e2; color: #991b1b; }

.info-direita { text-align: right; }
.valor.positivo { color: #10b981; }
.valor.negativo { color: #ef4444; }

.btn-excluir {
    background: none; border: none; cursor: pointer; font-size: 14px; opacity: 0.5;
}
.btn-excluir:hover { opacity: 1; }

#modal-overlay {
    position: fixed; top: 0; left: 0;
    width: 100%; height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex; justify-content: center; align-items: center;
    z-index: 1000;
}
.modal-box {
    background: white; padding: 25px; border-radius: 15px; width: 90%; max-width: 400px;
}

/* Estilos para o Modal do Gráfico */
.modal-grafico-box {
    max-width: 500px;
    text-align: center;
}
.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
}
.btn-fechar-x {
    background: none;
    border: none;
    font-size: 20px;
    cursor: pointer;
    color: #666;
}
.grafico-container-modal {
    position: relative;
    height: 300px;
    width: 100%;
}

.campo { margin-bottom: 15px; display: flex; flex-direction: column; }
.campo input, .campo select { padding: 10px; border: 1px solid #ddd; border-radius: 8px; }
.botoes-modal { display: flex; gap: 10px; margin-top: 20px; }
#btn-salvar { flex: 1; background-color: #FFAAEA; color: white; border: none; padding: 12px; border-radius: 8px; font-weight: bold; cursor: pointer; }
#btn-cancelar { flex: 1; background-color: transparent; border: 1px solid #ccc; color: #666; padding: 12px; border-radius: 8px; cursor: pointer; }
</style>
