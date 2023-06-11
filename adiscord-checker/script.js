function send() {
  let myTable = []
  for (let i = 0; i < document.getElementsByTagName("table").length; i++) {
    myTable[i] = {}

    const childrens = document.getElementsByTagName("table")[i].childNodes
    childrens.forEach((children) => {
      switch (children.nodeName) {
        case "TBODY": {
          myTable[i].childrens = {}

          const childrensBody = children.childNodes
          childrensBody.forEach((children, index) => {
            myTable[i].childrens[index] = []

            const childrensTd = children.childNodes
            childrensTd.forEach((children) => {
              myTable[i].childrens[index].push(children.textContent)
            })
          })

          return
        }

        case "THEAD": {
          myTable[i].fields = []

          const childrensBody = children.childNodes
          childrensBody.forEach((children) => {
            const childrensTh = children.childNodes
            childrensTh.forEach((children) => {
              myTable[i].fields.push(children.textContent)
            })
          })

          return
        }
      }
    })
  }

  return JSON.stringify(myTable)
}
