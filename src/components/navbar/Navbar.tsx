/* eslint-disable jsx-a11y/anchor-is-valid */
import React, { useState } from 'react';
import {
  createStyles,
  Navbar as MNavbar,
  Group,
  Code,
  getStylesRef,
  rem,
  Avatar,
} from '@mantine/core';
import { IconSwitchHorizontal, IconLogout } from '@tabler/icons-react';
import config from '@Config';
import { useNavigate } from 'react-router-dom';
import ppLogo from '../../assets/logo/pp-logo.jpg';
import { RouteData } from '../../routes/routes';

const useStyles = createStyles((theme) => ({
  header: {
    paddingBottom: theme.spacing.md,
    marginBottom: `calc(${theme.spacing.md} * 1.5)`,
    borderBottom: `${rem(1)} solid ${
      theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
    }`,
  },

  footer: {
    paddingTop: theme.spacing.md,
    marginTop: theme.spacing.md,
    borderTop: `${rem(1)} solid ${
      theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
    }`,
  },

  link: {
    ...theme.fn.focusStyles(),
    display: 'flex',
    alignItems: 'center',
    textDecoration: 'none',
    fontSize: theme.fontSizes.sm,
    color:
      theme.colorScheme === 'dark'
        ? theme.colors.dark[1]
        : theme.colors.gray[7],
    padding: `${theme.spacing.xs} ${theme.spacing.sm}`,
    borderRadius: theme.radius.sm,
    fontWeight: 500,

    '&:hover': {
      backgroundColor:
        theme.colorScheme === 'dark'
          ? theme.colors.dark[6]
          : theme.colors.gray[0],
      color: theme.colorScheme === 'dark' ? theme.white : theme.black,

      [`& .${getStylesRef('icon')}`]: {
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
      },
    },
  },

  linkIcon: {
    ref: getStylesRef('icon'),
    color:
      theme.colorScheme === 'dark'
        ? theme.colors.dark[2]
        : theme.colors.gray[6],
    marginRight: theme.spacing.sm,
  },

  linkActive: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({
        variant: 'light',
        color: theme.primaryColor,
      }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor })
        .color,
      [`& .${getStylesRef('icon')}`]: {
        color: theme.fn.variant({ variant: 'light', color: theme.primaryColor })
          .color,
      },
    },
  },
}));

type NavItemProps = {
  link: string;
  label: string;
  icon: React.ComponentType<any>;
  active: boolean;
  onClick: (e: React.MouseEvent<HTMLAnchorElement>) => void;
};

const NavItem: React.FC<NavItemProps> = ({
  link,
  label,
  icon,
  active,
  onClick,
}: NavItemProps) => {
  const { classes, cx } = useStyles();
  return (
    <a
      className={cx(classes.link, {
        [classes.linkActive]: active,
      })}
      href={link}
      key={label}
      onClick={(e) => onClick(e)}
    >
      {React.createElement(icon, { className: classes.linkIcon, stroke: 1.5 })}
      <span>{label}</span>
    </a>
  );
};

type NavbarProps = {
  initialRoutes: RouteData[];
};

const Navbar: React.FC<NavbarProps> = ({ initialRoutes }: NavbarProps) => {
  const { classes } = useStyles();
  const [active, setActive] = useState(initialRoutes[0].label || '');
  const navigate = useNavigate();

  const links = initialRoutes.map((route) => (
    <NavItem
      key={route.label}
      link={route.path}
      label={route.label}
      icon={route.icon}
      active={route.label === active}
      onClick={(e) => {
        e.preventDefault();
        navigate(route.path);
        setActive(route.label);
      }}
    />
  ));
  return (
    <MNavbar height="100%" width={{ sm: 300 }} p="md">
      <MNavbar.Section grow>
        <Group className={classes.header} position="apart">
          <Avatar src={ppLogo} />
          <Code sx={{ fontWeight: 700 }}>{config.appVersion}</Code>
        </Group>
        {links}
      </MNavbar.Section>

      <MNavbar.Section className={classes.footer}>
        <NavItem
          link="#"
          label="อะไรสักอย่าง"
          icon={IconSwitchHorizontal}
          active={false}
          onClick={() => {}}
        />
        <NavItem
          link="#"
          label="อะไรสักอย่าง"
          icon={IconLogout}
          active={false}
          onClick={() => {}}
        />
      </MNavbar.Section>
    </MNavbar>
  );
};

export default Navbar;
