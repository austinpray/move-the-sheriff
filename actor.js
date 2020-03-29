const React = require('react');
const { Box, Color, Text } = require('ink')

/**
 * Sherriff
 */
const Sherriff = props => <Actor {...props} model={`🤠`} />

/**
 * Cow
 */
const Cow = props => <Actor {...props} model={`🐄`} />

/**
 * Grass
 */
const Grass = props => <Actor {...props} model={`░`} bg={'#22863a'} color={'#FFFFFF'} />

/**
 * Actor
 */
const Actor = ({ bg, color, model, width, height }) => {
  return (
    <Box width={width || 1} height={height || 1}>
      <Color bgHex={bg || 'transparent'} color={color || '#FFFFFF'}>
        <Text>{model}</Text>
      </Color>
    </Box>
  )
}

module.exports = {
  Actor,
  Cow,
  Grass,
  Sherriff,
}
