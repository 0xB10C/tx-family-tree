function loadFromURL(url) {
  console.log("loading JSON from ", url);
  fetch(url)
    .then((res) => res.json())
    .then((block) => {
      console.log("processing block", block);
      draw(processBlock(block));
    })
    .catch((err) => {
      throw err;
    });
}


function processBlock(block) {
  let nodes = [];
  let knownTxids = new Set();
  let links = [];

  for(const tx of block.tx) {
    nodes.push({ "id": tx.txid, "group": 1, })
    knownTxids.add(tx.txid)
    for(const vin of tx.vin) {
      if(knownTxids.has(vin.txid)) {
        links.push({ "source": vin.txid, "target": tx.txid, "value": 0 })
      }
    }
  }
  return { "nodes": nodes, "links": links };
}

document.getElementById('selectFiles').addEventListener('change', function(e) {
  if (e.target.files[0]) {
  var fr = new FileReader();
    fr.onload = function (e) {
    let block = JSON.parse(e.target.result);
    draw(processBlock(block));
  };
  var files = document.getElementById("selectFiles").files;

  fr.readAsText(files.item(0));
  }
});

