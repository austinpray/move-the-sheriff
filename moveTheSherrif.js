const React = require('react');
const {Box, Text, Color, useInput, render} = require('ink');
const BigText = require('ink-big-text');
const useStdoutDimensions = require('ink-use-stdout-dimensions');

/**
 * Move the Sherrif
 */
const MoveTheSherrif = () => {
  const [ windowX, windowY ] = useStdoutDimensions();
  const [x, setX] = React.useState(windowX / 2)
  const [y, setY] = React.useState((windowY - 20) / 2)

  useInput((input, key) => {
    const height = ((windowY - 20) / 1);

    if (key.upArrow || input == "w") {
      if (y - 1 < 1) {
        setY(height - 1)
      } else {
        setY(y - 1)
      }
    }

    if (key.downArrow || input == "s") {
      if (y + 1 > height) {
        setY(1)
      } else {
        setY(y + 1)
      }
    }

    if (key.leftArrow || input == "a") {
      if (x - 1 < 1) {
        setX(windowX - 1)
      } else {
        setX(x - 1)
      }
    }

    if (key.rightArrow || input == "d") {
      if (x + 1 > windowX) {
        setX(1)
      } else {
        setX(x + 1)
      }
    }
  })

  return (
    <Box flexDirection="column">
      <BigText text={'Move the\nsheriff'} align={'center'} />

      <Box flexDirection="column">
        <Box marginTop={y} marginLeft={x}>🤠</Box>
      </Box>
    </Box>
  )
}

render(<MoveTheSherrif/>);
