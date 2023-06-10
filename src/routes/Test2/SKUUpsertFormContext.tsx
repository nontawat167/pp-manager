import { PropsWithChildren } from 'react';
import { FormProvider, SubmitHandler, useForm } from 'react-hook-form';

type SKUUpsertFormInput = {
  name: string;
  price: number;
  type: string;
};

type SKUUpsertFormProviderProps = {
  initValue?: SKUUpsertFormInput;
  onSubmit: SubmitHandler<SKUUpsertFormInput>;
};

const SKU_UPSERT_FORM_INPUT_DEFAULT_VALUES = {
  name: '',
  price: 0,
  type: '',
};

const SKUUpsertFormProvider = ({
  children,
  initValue = SKU_UPSERT_FORM_INPUT_DEFAULT_VALUES,
  onSubmit,
}: PropsWithChildren<SKUUpsertFormProviderProps>) => {
  const formContext = useForm<SKUUpsertFormInput>({
    defaultValues: initValue ?? SKU_UPSERT_FORM_INPUT_DEFAULT_VALUES,
    mode: 'onTouched',
  });
  return (
    <FormProvider {...formContext}>
      <form onSubmit={formContext.handleSubmit(onSubmit)} noValidate>
        {children}
      </form>
    </FormProvider>
  );
};

export { SKUUpsertFormProvider };
export type { SKUUpsertFormInput };
