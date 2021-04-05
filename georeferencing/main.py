from qgis.core import (
    QgsProject,
    QgsPathResolver
)

from qgis.gui import (
    QgsLayerTreeMapCanvasBridge,
)

project = QgsProject.instance()
project.read("IVand_K08_F2_T53_H1.qgs")
print(project.filename)
