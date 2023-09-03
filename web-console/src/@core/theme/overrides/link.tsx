
import NextLink, { LinkProps } from 'next/link'
import { forwardRef } from 'react'

// https://mui.com/material-ui/guides/routing/
// Using Next.js link to leverage its SPA-like behaviour
const LinkBehaviour = forwardRef<any, LinkProps>(
  (props, ref) => (<NextLink ref={ref} {...props}/>)
)

export default {
  MuiLink: {
    styleOverrides: {
      root: {
        textDecoration: 'none'
      }
    },
    defaultProps: {
      // https://stackoverflow.com/questions/66226576/using-the-material-ui-link-component-with-the-next-js-link-component
      component: LinkBehaviour
    }
  }
}
