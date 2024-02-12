
function draw(data) {
  g = ForceGraph()
    (document.getElementById('graph'))
      .graphData(data)
      .nodeId('id')
      .nodeVal('val')
      .nodeLabel('id')
      .linkDirectionalArrowLength(6)
      .d3Force('center', null)
      .d3Force('x', d3.forceX())
      .d3Force('y', d3.forceY())
      .backgroundColor('#101020')
      .onNodeClick(node => window.open(`https://mempool.space/tx/${node.id}`, '_blank'))
      .nodeAutoColorBy("version")
      .linkLabel("intype")
      .linkAutoColorBy("intype")
      .onNodeDragEnd(node => {
        node.fx = node.x;
        node.fy = node.y;
      })
      .linkSource('source')
      .linkTarget('target');
}
