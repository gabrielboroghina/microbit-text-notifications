const update = document.querySelector('#update_button')
update.addEventListener('click', _ => {

    const nameString = document.getElementById('namePut').value
    const notificationString = document.getElementById('notificationPut').value
    fetch('/notifications', {
        method: 'put',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            name: nameString,
            notification: notificationString
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
    const nameString = document.getElementById('nameDelete').value
    fetch('/notifications', {
        method: 'delete',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            name: nameString
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