import { Container, Divider } from '@mantine/core';
import React, { ReactElement } from 'react';

type Props = {
  title: string;
  children: ReactElement;
};

const MainContent: React.FC<Props> = ({ title, children }: Props) => {
  return (
    <Container fluid pt={20} sx={() => ({ width: '100%' })}>
      <div>{title}</div>
      <Divider my="sm" />
      {children}
      <Divider my="sm" />
      <div>footer</div>
    </Container>
  );
};

export default MainContent;
