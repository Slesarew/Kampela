package fi.zymologia.siltti

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
/*
@Composable
fun ScanProgressBar(
    frames: State<Frames?>,
    resetScan: () -> Unit
) {
    val frontColor = MaterialTheme.colors.onSecondary

    frames.value?.let {
        Surface(
            color = MaterialTheme.colors.background,
            shape = MaterialTheme.shapes.modal
        ) {
            Column(
                modifier = Modifier.padding(20.dp)
            ) {
                HeadingOverline(text = "PARSING MULTIPART DATA")
                Canvas(
                    modifier = Modifier
                        .height(24.dp)
                        .fillMaxWidth()
                ) {
                    drawRect(
                        frontColor,
                        Offset.Zero.copy(x = 0.dp.toPx(), y = 8.dp.toPx()),
                        Size(width = this.size.width, height = 8.dp.toPx()),
                        style = Stroke()
                    )
                    drawRect(
                        frontColor,
                        Offset.Zero.copy(x = 0.dp.toPx(), y = 8.dp.toPx()),
                        Size(
                            //total is never zero
                            width = this.size.width * it.current.toFloat().div(it.total.toFloat()),
                            height = 8.dp.toPx()
                        )
                    )
                }
                Text(
                    "From " + it.current + " / " + it.total + " captured frames",
                    style = MaterialTheme.typography.subtitle1,
                    color = MaterialTheme.colors.Text600
                )
                Text(
                    "Please hold still",
                    style = MaterialTheme.typography.subtitle2,
                    color = MaterialTheme.colors.Text400
                )
                BigButton(
                    text = "Start over",
                    action = { resetScan() }
                )
            }
        }
    }
}*/