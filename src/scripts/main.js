const filterInput = document.getElementById('filterInput');

function filterTable() {
  const inputText = filterInput.value.trim().toLowerCase()
  const tableRows = document.querySelectorAll('tbody tr');

  tableRows.forEach((row) => {
    const cells = row.querySelectorAll('td');
    let rowMatches = false;

    cells.forEach((cell) => {
      const cellText = cell.textContent.toLowerCase();

      if (cellText.includes(inputText)) {
        rowMatches = true;
      }
    })

    if (rowMatches) {
      row.style.display = '';
    } else {
      row.style.display = 'none';
    }
  })
}

filterInput.addEventListener('input', filterTable);