
const input_uno = document.getElementById("input_uno");
const input_dos = document.getElementById("input_dos");
const input_category = document.getElementById("customer")

const command_uno = document.getElementById("command_uno")
const command_dos = document.getElementById("command_dos")
const command_tres = document.getElementById("command_tres")

command_uno.addEventListener('click', function () {
    console.log(input_uno.value)
    invoke("command_uno", {
        uno: input_uno.value,
        categoryType: input_category.value
    })
})

command_dos.addEventListener('click', function () {
    console.log(input_dos.value)
    invoke("command_dos", {
        dos: input_dos.value,
        codebar: Number(input_dos.value),
        amount: 2,
        usd: 350
    })
})

command_tres.addEventListener('click', function () {
    console.log('tres')
    invoke("command_tres", {
        tres: "3"
    })
})