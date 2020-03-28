import React, { useState } from 'react'
import { Box, Text, Color, useInput } from 'ink'
import BigText from 'ink-big-text'
import useStdoutDimensions from 'ink-use-stdout-dimensions'

/**
 * Move the Sherrif
 */
const MoveTheSherrif = () => {
  const [ windowX, windowY ] = useStdoutDimensions();
  const [x, setX] = useState(windowX / 2)
  const [y, setY] = useState((windowY - 20) / 2)

  useInput((input, key) => {
    const height = ((windowY - 20) / 1);

    if (key.upArrow) {
      if (y - 1 < 1) {
        setY(height - 1)
      } else {
        setY(y - 1)
      }
    }

    if (key.downArrow) {
      if (y + 1 > height) {
        setY(1)
      } else {
        setY(y + 1)
      }
    }

    if (key.leftArrow) {
      if (x - 1 < 1) {
        setX(windowX - 1)
      } else {
        setX(x - 1)
      }
    }

    if (key.rightArrow) {
      if (x + 1 > windowX) {
        setX(1)
      } else {
        setX(x + 1)
      }
    }
  })

  return (
    <Box flexDirection="column">
      <BigText text={'Move the\nsherrif'} align={'center'} />

      <Box flexDirection="column">
        <Box marginTop={y} marginLeft={x}>🤠</Box>
      </Box>
    </Box>
  )
}

export default MoveTheSherrif;
