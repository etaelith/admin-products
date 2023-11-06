const { invoke } = window.__TAURI__.tauri;
const dialog = window.__TAURI__.dialog;

// lock & unlock
const radioButtons = document.querySelectorAll('input[type="radio"]');
const dataBoletas = document.querySelectorAll('[data-boletas]');
const div1 = document.querySelector('[data-label="1"]');
const div2 = document.querySelector('[data-label="2"]');
const div3 = document.querySelector('[data-label="3"]');
// Boleta 
const buttonCreate = document.getElementById("create")
const billingType = document.getElementById("customer")
const inputDNI = document.getElementById("dni")
//Search boleta
const valueIdTable = document.getElementById("testeando")
const searchButton = document.getElementById('search')

const tableIdPut = document.getElementById("tableRowId")
//agree
const buttonAgree = document.getElementById("agree")
const input_dos = document.getElementById("input_dos");
const idProduct = document.getElementById("productId")
const amountProduct = document.getElementById("amount")

// Cancel Sell
const cancelButton = document.getElementById('cancel')
/* Calculate Total */
const buttonCalculate = document.getElementById("calculate")
const totalShow = document.getElementById("totalAmount")
/* Sell */
const buttonSell = document.getElementById("sell")
//add events

const scanButton = document.getElementById('scanner')


radioButtons.forEach(radioButton => {
    radioButton.addEventListener('change', function () {
        if (div2.classList.contains("desactivar-div")) {
            div2.classList.remove('desactivar-div')
        } else {
            console.log('nope')
        }
        const selectedValue = this.value;
        dataBoletas.forEach(dataBoleta => {
            if (dataBoleta.getAttribute('data-boletas') === selectedValue) {
                dataBoleta.classList.remove('desactivar-div');
            } else {
                dataBoleta.classList.add('desactivar-div');
            }
        });
    });
});
// Select boleta & Create boleta
buttonCreate.addEventListener('click', function () {
    invoke("command_uno", {
        categoryType: billingType.value,
        dni: Number(inputDNI.value)
    }).then((result) => {
        if (result !== null) {
            console.log('Last insert row: ', result)
            tableIdPut.innerText = '';
            tableIdPut.innerText = result;
            next()
        } else {
            console.log('No se inserto ninguna fila')
        }
    }).catch((error) => {
        console.error('Error en la funcion rust: ', error)
    })
})
/* Buscar Boleta */
searchButton.addEventListener('click', async function () {
    await invoke("search_boleta", {
        idRow: Number(valueIdTable.value)
    }).then((result) => {
        if (result.success === true) {
            tableIdPut.innerText = valueIdTable.value;
            next()
        } else {
            dialog.message(result.error_message, "Error al buscar")
            valueIdTable.value = '';
            valueIdTable.focus()
        }
    }).catch((error) => {
        dialog.message(error, "Error al buscar")
        console.log(error)
    })
})
/* Agregar Producto a la boleta */
buttonAgree.addEventListener("click", async function () {
    let codebar_clean = idProduct.value.trim();
    if (codebar_clean !== "") {
        await invoke('command_dos', {
            codebar: Number(idProduct.value),
            amount: Number(amountProduct.value),
            usd: 350,
            idRowTable: Number(tableIdPut.innerText)
        }).then((result) => {
            showItems(result)
            idProduct.value = '';
            amountProduct.value = '';
        }).catch((error) => {
            dialog.message(error, 'Error al procesar')
            console.log(error)
        })
    }

})
/* Calculate Total */
buttonCalculate.addEventListener("click", function () {
    const rows = document.querySelectorAll('.item-row'); // Selecciona todas las filas con la clase "item-row"
    let totalArsSum = 0;

    rows.forEach(row => {
        // La celda totalArsCell está en la posición 6 de la fila
        const totalArsValue = parseFloat(row.textContent);
        if (!isNaN(totalArsValue)) {
            totalArsSum += totalArsValue;
        }
    });
    totalShow.innerText = `$ ${totalArsSum}`
})
/* Cancelar Venta */
cancelButton.addEventListener('click', async function () {
    if (tableIdPut.textContent === 'NN') return
    const result = await dialog.confirm("Estas seguro que deseas", "Cancelar Venta")
    if (result) {
        invoke('cancel_selldelete', {
            buyerId: Number(tableIdPut.innerText)
        }).then(() => {
            const table = document.querySelector("table tbody")
            table.innerHTML = '';
            tableIdPut.innerText = 'NN';
            inputDNI.value = '';
            idProduct.value = '';
            amountProduct.value = '';
            back()
        })
    } else {
        console.log('nou')
    }
})

/* Sell Total */
buttonSell.addEventListener('click', async function () {
    const table = document.querySelector("table tbody")
    if (tableIdPut.textContent === 'NN') return
    if (table.childElementCount > 0) {
        const result = await dialog.confirm("Estas seguro que deseas", "Confirmar la venta?", "Confirmar Venta")
        if (result) {
            invoke("command_tres", {
                buyerId: Number(tableIdPut.innerText)
            }).then((result) => {
                if (result.success === true) {
                    const table = document.querySelector("table tbody")
                    table.innerHTML = ''
                    tableIdPut.innerText = 'NN';
                    back()
                } else {
                    dialog.message(error, "Error transaction")
                }
            }).catch((error) => {
                dialog.message(error, "Error consulta")

            })
        } else {
            console.log('Nope')
        }
    }
})

function showItems(item) {
    const table = document.querySelector("table tbody")
    const row = table.insertRow();
    const idCell = row.insertCell(0);
    const idProductCell = row.insertCell(1);
    const nameCell = row.insertCell(2);
    const amountCell = row.insertCell(3);
    const arsCell = row.insertCell(4);
    const usdCell = row.insertCell(5)
    const totalArsCell = row.insertCell(6)
    const editCell = row.insertCell(7);
    editCell.classList.add('icons-svg');
    const deleteIcon = document.createElement('img');
    deleteIcon.src = './assets/delete.svg';
    deleteIcon.addEventListener('click', deleteFromTable)
    const configIcon = document.createElement('img');
    configIcon.src = './assets/config.svg';
    configIcon.addEventListener('click', function () {
        console.log('config')
    })
    editCell.appendChild(deleteIcon);
    editCell.appendChild(configIcon);
    idCell.textContent = item.id
    idProductCell.textContent = item.codebar
    nameCell.textContent = item.name
    amountCell.textContent = item.amount
    arsCell.textContent = item.price
    usdCell.textContent = item.usd_value
    totalArsCell.textContent = item.total
    totalArsCell.classList.add('item-row')

}
async function deleteFromTable() {
    const row = this.closest('tr');
    const codebarCell = row.querySelector("td:nth-child(2)");
    const codebar = codebarCell.textContent;
    console.log(`buyerId: ${tableIdPut.innerText}, \n codebar: ${codebar}`)
    const result = await dialog.confirm('Estas seguro que deseas borrar este item?', 'Borrar item')
    if (result) {
        const response = await invoke('delete_item_specific', {
            buyerId: Number(tableIdPut.innerText),
            codebar: Number(codebar)
        });

        if (response && response.success) {
            row.remove();
        } else {
            console.log('Error al borrar el item');
        }
    } else {
        console.log('nope')
    }
}
async function fitPrice(percent) {
    try {
        await invoke('update_prices_db', {
            percent
        })
        console.log('Precios actualizados exitosamente')
    } catch (error) {
        console.error('Error al actualizar los precios: ', error)
    }

}


function next() {
    console.log('next')
    div1.classList.add('desactivar-div')
    div2.classList.add('desactivar-div')
    div3.classList.remove('desactivar-div')
}

function back() {
    console.log('back')
    div1.classList.remove('desactivar-div')
    div3.classList.add('desactivar-div')
}