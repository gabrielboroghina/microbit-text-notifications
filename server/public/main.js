const update = document.querySelector('#update_button')
update.addEventListener('click', _ => {
    fetch('/notifications', {
        method: 'put',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            name: 'a',
            notification: 'r'
        })
    })
    .then(res => {
        if (res.ok) return res.json()
    })
    .then(response => {
        console.log(response)
    })
})

const deleteButton = document.querySelector('#delete-button')
const messageDiv = document.querySelector('#message')
deleteButton.addEventListener('click', _ => {
    fetch('/notifications', {
        method: 'delete',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            name: 'a'
        })
    })
    .then(res => {
        if (res.ok) return res.json()
    })
    .then(response => {
        messageDiv.textContent = response
        console.log(response)
    })
})