const { invoke } = window.__TAURI__.tauri
const idProduct = document.getElementById("productId")
const amountProduct = document.getElementById("amount")
const buttonAgree = document.getElementById("agree")
const buttonSell = document.getElementById("sell")
const billingType = document.getElementById("customer")
const tableRows = document.querySelectorAll(".row-item")
const buttonCalculate = document.getElementById("calculate")
const total = document.getElementById("totalAmount")
/*Check exist & stock */
idProduct.addEventListener("input", function () {
    const trimmedValue = idProduct.value.trim();
    if (trimmedValue !== "") {
        console.log(trimmedValue)
    }
})

/*  Agree item cart */
buttonAgree.addEventListener("click", function () {
    console.log(idProduct.value, Number(amountProduct.value))
})

/* Calculate total */

let amountTotal;
buttonCalculate.addEventListener("click", function () {
    let price_total = 0;
    for (let i = 0; i < tableRows.length; i++) {
        const amount = parseFloat(tableRows[i].querySelector("td:nth-child(7)").textContent.replace("$", "").trim());
        price_total = price_total + amount;
    }
    total.innerText = `$ ${price_total}`
    amountTotal = price_total;
    invoke("create_bill", {
        categoryType: billingType.value
    })
})

/* Payload */


buttonSell.addEventListener('click', async function () {
    /* buttonCalculate.click();
    console.log("Billing Type: ", billingType.value, " ,Total Amount: ", amountTotal) */
    const hasRecords = await invoke("check_buyer_records_db");
    if (hasRecords === true) {
        console.log('La tabla tiene registros')
    } else {
        console.log('la tabla NO tiene registros')
    }
})